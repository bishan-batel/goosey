use crate::file::trace::Trace;
use crate::ir::visibility::Visibility;
use crate::lexer::keyword::Keyword;
use crate::lexer::token::Operator;
use crate::parser::ast::data::{UnparsedVariableInfo, UnvalidatedType};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::parser::ast::function::{UnparsedFunctionPrototype, UnvalidatedFunctionExpression};
use crate::parser::ast::top_level::UnparsedTopLevel;
use crate::parser::error::{ParserError, ParserResult};
use crate::parser::modules::expression_parser::ExpressionParser;
use crate::parser::parser::Parser;

pub struct TopLevelParser;


impl TopLevelParser {
    pub fn parse_top_level(p: &mut Parser) -> ParserResult<Option<UnparsedTopLevel>> {
        let passes = [Self::parse_function];

        for pass in passes {
            if let Some(statement) = pass(p)? {
                return Ok(Some(statement));
            }
        }
        Ok(None)
    }

    fn parse_function(p: &mut Parser) -> ParserResult<Option<UnparsedTopLevel>> {
        let start = p.position();

        let stepped = if p.has_keyword(Keyword::Public) {
            p.advance();
            true
        } else { false };

        if !p.has_keyword(Keyword::Fun) {
            // rollback to before visibility keyword
            if stepped {
                p.rollback();
            }
            return Ok(None);
        }

        // rollback to before visibility keyword
        if stepped {
            p.rollback();
        }

        let proto = Self::consume_function_prototype(p)?;

        let body = if p.has_operator(Operator::ThickRightArrow) {
            p.advance();
            ExpressionParser::consume_expression(p)?.into()
        } else {
            if let Some(block) = ExpressionParser::parse_block(p)? {
                block.into()
            } else {
                return Err(ParserError::ExpectedFunctionBody(p.trace_from(start)));
            }
        };

        Ok(Some(UnparsedTopLevel::FunctionDefinition {
            proto,
            body,
            trace: p.trace_from(start),
        }))
    }

    fn consume_function_prototype(p: &mut Parser) -> ParserResult<UnparsedFunctionPrototype> {
        let visibility = if p.has_keyword(Keyword::Public) {
            p.advance();
            Visibility::Public
        } else {
            Visibility::Private
        };

        p.expect_keyword(Keyword::Fun)?;

        let name = p.consume_identifier()?;


        let mut arguments = vec![];

        p.expect_operator(Operator::ParenOpen)?;

        while !p.has_operator(Operator::ParenClose) && !p.is_eof() {
            let mutable = if p.has_keyword(Keyword::Mut) {
                p.advance();
                true
            } else {
                false
            };

            let arg_name = p.consume_identifier()?;

            p.expect_operator(Operator::Colon)?;

            let arg_ty = p.consume_type()?;

            arguments.push(UnparsedVariableInfo {
                ident: arg_name,
                ty: arg_ty,
                mutable,
            });

            if !p.has_operator(Operator::Comma) {
                break;
            }

            p.advance();
        }
        p.expect_operator(Operator::ParenClose)?;

        let returns = if p.has_operator(Operator::Colon) {
            p.expect_operator(Operator::Colon)?;
            p.consume_type()?
        } else {
            UnvalidatedType::Unit
        };

        Ok(UnparsedFunctionPrototype {
            name,
            arguments,
            returns,
            visibility,
        })
    }
}
