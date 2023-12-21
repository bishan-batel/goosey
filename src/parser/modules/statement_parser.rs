use crate::lexer::keyword::Keyword;
use crate::lexer::token::Operator;
use crate::parser::ast::data::{UnvalidatedVariableInfo, UnvalidatedType};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::parser::ast::function::UnvalidatedFunctionExpression;
use crate::parser::ast::operations::UnaryOperator;
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
            Self::parse_let_statement,
            Self::parse_while,
            Self::parse_if,
            Self::parse_return_statement
        ];

        for pass in passes {
            if let Some(expr) = pass(p)? {
                return Ok(expr);
            }
        }

        Ok(ExpressionParser::consume_expression(p)?.into())
    }

    pub fn parse_return_statement(p: &mut Parser) -> StatementParseResult {
        if !p.has_keyword(Keyword::Return) {
            return Ok(None);
        }
        p.advance();

        let expr = if p.has_keyword(Keyword::Unit) {
            p.advance();
            None
        } else {
            Some(ExpressionParser::consume_expression(p)?)
        };


        Ok(Some(UnvalidatedFunctionExpression::Return(expr)))
    }

    pub fn parse_while(p: &mut Parser) -> StatementParseResult {
        let until = if p.has_keyword(Keyword::While) {
            false
        } else if p.has_keyword(Keyword::Until) {
            true
        } else {
            return Ok(None);
        };
        let start = p.position();

        p.advance();

        let condition = ExpressionParser::consume_expression(p)?;

        let condition = if until {
            UnvalidatedExpression::Unary {
                trace: condition.trace().clone(),
                expr: Box::new(condition),
                op: UnaryOperator::Not,
            }
        } else { condition };

        if let Some(expr) = ExpressionParser::parse_block(p)? {
            Ok(Some(UnvalidatedFunctionExpression::While {
                condition,
                then: Box::new(expr),
                trace: p.trace_from(start),
            }))
        } else {
            Err(ParserError::ExpectedOperator(
                Operator::CurlyOpen,
                p.trace_from(start),
            ))
        }
    }

    pub fn parse_if(p: &mut Parser) -> StatementParseResult {
        let unless = if p.has_keyword(Keyword::If) {
            false
        } else if p.has_keyword(Keyword::Unless) {
            true
        } else {
            return Ok(None);
        };
        let start = p.position();

        p.advance();

        let condition = ExpressionParser::consume_expression(p)?;

        let condition = if unless {
            UnvalidatedExpression::Unary {
                op: UnaryOperator::Not,
                trace: condition.trace().clone(),
                expr: Box::new(condition),
            }
        } else { condition };

        if let Some(expr) = ExpressionParser::parse_block(p)? {
            Ok(Some(UnvalidatedFunctionExpression::If {
                condition,
                then: Box::new(expr),
                trace: p.trace_from(start),
            }))
        } else {
            Err(ParserError::ExpectedOperator(
                Operator::CurlyOpen,
                p.trace_from(start),
            ))
        }
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
            variable: UnvalidatedVariableInfo {
                ident,
                ty,
                mutable,
            },
            initial,
            trace: p.trace_from(start),
        }))
    }
}
