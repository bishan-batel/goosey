use crate::file::trace::Trace;
use crate::ir::visibility::Visibility;
use crate::lexer::keyword::Keyword;
use crate::lexer::token::Operator;
use crate::parser::ast::data::{UnvalidatedVariableInfo, UnvalidatedType};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::parser::ast::function::{UnvalidatedFunctionPrototype, UnvalidatedFunctionExpression};
use crate::parser::ast::r#struct::{UnvalidatedEnumData, UnvalidatedEnumVariant, UnvalidatedProperty, UnvalidatedStructProperty, UnvalidatedStructPrototype};
use crate::parser::ast::top_level::UnvalidatedTopLevel;
use crate::parser::error::{ParserError, ParserResult};
use crate::parser::modules::expression_parser::ExpressionParser;
use crate::parser::parser::Parser;

pub struct TopLevelParser;


impl TopLevelParser {
    pub fn parse_top_level(p: &mut Parser) -> ParserResult<Option<UnvalidatedTopLevel>> {
        let passes = [
            Self::parse_function,
            Self::parse_import,
            Self::parse_struct
        ];

        for pass in passes {
            if let Some(statement) = pass(p)? {
                return Ok(Some(statement));
            }
        }
        Ok(None)
    }

    fn parse_import(p: &mut Parser) -> ParserResult<Option<UnvalidatedTopLevel>> {
        if !p.has_keyword(Keyword::Import) {
            return Ok(None);
        }

        let start = p.position();
        p.advance();

        let namespace = p.consume_symbol()?;

        Ok(Some(UnvalidatedTopLevel::Import {
            namespace,
            // TODO allow star imports
            star: false,
            trace: p.trace_from(start),
        }))
    }

    fn parse_struct(p: &mut Parser) -> ParserResult<Option<UnvalidatedTopLevel>> {
        let start = p.position();

        let public = if p.has_keyword(Keyword::Public) {
            p.advance();
            true
        } else {
            false
        };

        if !p.has_keyword(Keyword::Struct) {
            // rollback to before visibility keyword
            if public {
                p.rollback();
            }
            return Ok(None);
        }

        p.advance();

        let identifier = p.consume_identifier()?;

        let mut properties = vec![];
        p.expect_operator(Operator::CurlyOpen)?;

        while !p.is_eof() && !p.has_operator(Operator::CurlyClose) {
            let visibility = if p.has_keyword(Keyword::Public) {
                p.advance();
                Visibility::Public
            } else {
                Visibility::Private
            };

            let name = p.consume_identifier()?;
            p.expect_operator(Operator::Colon)?;

            let ty = p.consume_type()?;

            properties.push(UnvalidatedStructProperty {
                property: UnvalidatedProperty {
                    name,
                    ty,
                },
                visibility,
            })
        }


        p.expect_operator(Operator::CurlyClose)?;
        Ok(Some(UnvalidatedTopLevel::StructDefinition {
            proto: UnvalidatedStructPrototype {
                identifier,
                properties,
                visibility: if public { Visibility::Public } else { Visibility::Private },
            },
            trace: p.trace_from(start),
        }))
    }

    fn parse_enum(p: &mut Parser) -> ParserResult<Option<UnvalidatedTopLevel>> {
        let start = p.position();

        let public = if p.has_keyword(Keyword::Public) {
            p.advance();
            true
        } else {
            false
        };

        if !p.has_keyword(Keyword::Enum) {
            // rollback to before visibility keyword
            if public {
                p.rollback();
            }
            return Ok(None);
        }

        p.advance();

        let mut variants = vec![];
        p.expect_operator(Operator::CurlyOpen)?;

        while !p.is_eof() && !p.has_operator(Operator::CurlyClose) {
            let name = p.consume_identifier()?;

            variants.push(if p.has_operator(Operator::ParenOpen) {
                p.advance();

                let mut position = vec![];
                while !p.is_eof() && !p.has_operator(Operator::ParenClose) {
                    position.push(p.consume_type()?);

                    if p.has_operator(Operator::Comma) {
                        p.advance();
                        continue;
                    }
                    break;
                }
                p.expect_operator(Operator::ParenClose)?;

                UnvalidatedEnumVariant {
                    name,
                    data: Some(UnvalidatedEnumData::Positional(position)),
                }
            } else if p.has_operator(Operator::CurlyOpen) {
                p.advance();

                let mut properties = vec![];
                while !p.is_eof() && !p.has_operator(Operator::CurlyClose) {
                    let name = p.consume_identifier()?;
                    p.expect_operator(Operator::Colon)?;

                    let ty = p.consume_type()?;

                    properties.push(UnvalidatedProperty {
                        name,
                        ty,
                    })
                }
                p.expect_operator(Operator::CurlyClose)?;

                UnvalidatedEnumVariant {
                    name,
                    data: Some(UnvalidatedEnumData::StructLike(properties)),
                }
            } else {
                UnvalidatedEnumVariant {
                    name,
                    data: None,
                }
            })
        }


        p.expect_operator(Operator::CurlyClose)?;
        Ok(Some(UnvalidatedTopLevel::EnumDefinition {
            variants,
            trace: p.trace_from(start),
        }))
    }

    fn parse_function(p: &mut Parser) -> ParserResult<Option<UnvalidatedTopLevel>> {
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

        Ok(Some(UnvalidatedTopLevel::FunctionDefinition {
            proto,
            body,
            trace: p.trace_from(start),
        }))
    }

    fn consume_function_prototype(p: &mut Parser) -> ParserResult<UnvalidatedFunctionPrototype> {
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

            arguments.push(UnvalidatedVariableInfo {
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

        Ok(UnvalidatedFunctionPrototype {
            name,
            arguments,
            returns,
            visibility,
        })
    }
}
