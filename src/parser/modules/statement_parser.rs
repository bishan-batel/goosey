use crate::lexer::keyword::Keyword;
use crate::lexer::token::Operator;
use crate::parser::ast::data::{UnparsedVariableInfo, UnvalidatedType};
use crate::parser::ast::function::UnvalidatedFunctionExpression;
use crate::parser::error::{ParserError, ParserResult};
use crate::parser::modules::expression_parser::ExpressionParser;
use crate::parser::parser::Parser;

pub struct StatementParser;


type StatementParseResult = ParserResult<Option<UnvalidatedFunctionExpression>>;

impl StatementParser {
    pub fn consume_function_expression(
        p: &mut Parser
    ) -> ParserResult<UnvalidatedFunctionExpression> {
        let passes = [
            Self::parse_let_statement
        ];

        for pass in passes {
            if let Some(expr) = pass(p)? {
                return Ok(expr);
            }
        }

        Ok(ExpressionParser::consume_expression(p)?.into())
    }

    pub fn parse_let_statement(p: &mut Parser) -> StatementParseResult {
        if !p.has_keyword(Keyword::Let) {
            return Ok(None);
        }

        let start = p.position();

        p.advance();

        let mutable = if p.has_keyword(Keyword::Mut) {
            p.advance();
            true
        } else {
            false
        };

        let ident = p.consume_identifier()?;

        let ty = if p.has_operator(Operator::Colon) {
            p.advance();
            p.consume_type()?
        } else {
            UnvalidatedType::Implicit
        };

        let initial = if p.has_operator(Operator::Equals) {
            p.advance();
            Some(ExpressionParser::consume_expression(p)?)
        } else {
            None
        };

        Ok(Some(UnvalidatedFunctionExpression::Let {
            variable: UnparsedVariableInfo {
                ident,
                ty,
                mutable,
            },
            initial,
            trace: p.trace_from(start),
        }))
    }
}
