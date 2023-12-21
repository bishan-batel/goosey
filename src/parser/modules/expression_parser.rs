use crate::lexer::keyword::Keyword;
use crate::lexer::token::{Operator, Token, TokenData};
use crate::parser::ast::data::UnvalidatedType;
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::parser::ast::function::UnvalidatedFunctionExpression;
use crate::parser::ast::operations::{BinaryOperation, UnaryOperator};
use crate::parser::error::{ParserError, ParserResult};
use crate::parser::modules::statement_parser::StatementParser;
use crate::parser::parser::Parser;

pub struct ExpressionParser;

pub const ORDER_OF_OPERATIONS: &[&[Operator]] = &[
    &[], // factor / unreachable
    &[Operator::Multiply, Operator::Divide, Operator::Mod],
    &[Operator::Plus, Operator::Minus],
    &[Operator::BitShiftLeft, Operator::BitShiftRight],
    &[Operator::Less, Operator::Greater, Operator::LessEquals, Operator::GreaterEquals],
    &[Operator::Equals, Operator::NotEquals],
    &[Operator::BitAnd],
    &[Operator::Xor],
    &[Operator::BitOr],
    &[Operator::And],
    &[Operator::Or],
    &[
        Operator::ModAssigns,
        Operator::XorAssign,
        Operator::BitAndAssign,
        Operator::MultiplyAssign,
        Operator::MinusAssign,
        Operator::PlusAssign,
        Operator::DivideAssign,
        Operator::BitShiftLeftAssign,
        Operator::BitShiftRightAssign,
        Operator::AndAssign,
        Operator::OrAssign,
        Operator::Assign
    ],
];


impl ExpressionParser {
    pub fn parse_block(p: &mut Parser) -> ParserResult<Option<UnvalidatedExpression>> {
        if !p.has_operator(Operator::CurlyOpen) {
            return Ok(None);
        }

        p.advance();

        let mut body = vec![];
        let start = p.position();

        while !p.is_eof() && !p.has_operator(Operator::CurlyClose) {
            let statement = StatementParser::consume_function_expression(p)?;
            body.push(statement);
        }
        p.expect_operator(Operator::CurlyClose)?;

        Ok(Some(UnvalidatedExpression::Scope(body, p.trace_from(start))))
    }
    pub fn consume_expression(p: &mut Parser) -> ParserResult<UnvalidatedExpression> {
        Self::consume_binary_expression(p, ORDER_OF_OPERATIONS.len() - 1)
    }

    fn consume_factor(p: &mut Parser) -> ParserResult<UnvalidatedExpression> {
        let trace = p.curr().trace().clone();

        Ok(match p.curr().token().clone() {
            TokenData::StringLiteral(s) => {
                p.advance();
                UnvalidatedExpression::StringLiteral(s, trace)
            }
            TokenData::BoolLiteral(l) => {
                p.advance();
                UnvalidatedExpression::BoolLiteral(l, trace)
            }
            TokenData::F32Literal(l) => {
                p.advance();
                UnvalidatedExpression::F32Literal(l, trace)
            }
            TokenData::F64Literal(l) => {
                p.advance();
                UnvalidatedExpression::F64Literal(l, trace)
            }
            TokenData::I32Literal(l) => {
                p.advance();
                UnvalidatedExpression::I32Literal(l, trace)
            }
            TokenData::I64Literal(l) => {
                p.advance();
                UnvalidatedExpression::I64Literal(l, trace)
            }
            TokenData::Identifier(_) => {
                let start = p.position();
                let symbol = p.consume_symbol()?;

                if !p.has_operator(Operator::ParenOpen) {
                    UnvalidatedExpression::VariableReference {
                        symbol,
                        trace: p.trace_from(start),
                    }
                } else {
                    p.advance();
                    let mut arguments = vec![];

                    while !p.is_eof() && !p.has_operator(Operator::ParenClose) {
                        arguments.push(Self::consume_expression(p)?);

                        if p.has_operator(Operator::Comma) {
                            p.advance();
                            continue;
                        }
                        break;
                    }
                    p.expect_operator(Operator::ParenClose)?;

                    UnvalidatedExpression::FunctionCall {
                        symbol,
                        arguments,
                        trace: p.trace_from(start),
                    }
                }
            }

            // Operators
            TokenData::Operator(op) => match op {

                // Parenthetical
                Operator::ParenOpen => {
                    let start = p.position();

                    p.advance();
                    let expr = Self::consume_expression(p)?;
                    p.expect_operator(Operator::ParenClose)?;
                    UnvalidatedExpression::Parenthetical(
                        Box::new(expr),
                        p.trace_from(start),
                    )
                }

                Operator::Not | Operator::BitNot | Operator::Minus => {
                    let start = p.position();
                    p.advance();

                    UnvalidatedExpression::Unary {
                        expr: Box::new(Self::consume_expression(p)?),
                        op: UnaryOperator::try_from(op).expect("Non unary operator"),
                        trace: p.trace_from(start),
                    }
                }
                _ => return Err(ParserError::UnexpectedToken(p.curr().clone()))
            }

            _ => return {
                Err(ParserError::UnexpectedToken(p.curr().clone()))
            }
        })
    }

    fn consume_expression_cast(p: &mut Parser) -> ParserResult<UnvalidatedExpression> {
        let mut expr = Self::consume_factor(p)?;
        let start = p.position();

        while p.has_keyword(Keyword::As) {
            p.advance();

            expr = UnvalidatedExpression::Cast {
                expr: Box::new(expr),
                ty: p.consume_type()?,
                trace: p.trace_from(start),
            }
        }

        Ok(expr)
    }

    fn consume_binary_expression(
        p: &mut Parser,
        op_index: usize,
    ) -> ParserResult<UnvalidatedExpression> {
        // op precedence of 0 defaults to parsing a factor
        // (eg. literal or unary or parenthesis or function call, etc)
        if op_index == 0 {
            return Self::consume_expression_cast(p);
        }

        let valid_ops = ORDER_OF_OPERATIONS[op_index];

        let mut parse_operand = |p: &mut Parser| {
            Self::consume_binary_expression(p, op_index - 1)
        };

        // get the left hand side of the expression by parsing by the operator
        let mut lhs = parse_operand(p)?;

        let start = p.position();

        // exit loop if the next token is not an operator
        // with the precedence being looked for
        while let TokenData::Operator(op) = p.curr().token().clone() {
            if !valid_ops.contains(&op) { break; }
            p.advance();

            // turn the lhs to a binary operation between it and the next
            // expression of 1 less precedence
            lhs = UnvalidatedExpression::Binary {
                lhs: Box::new(lhs),
                op: BinaryOperation::try_from(op).expect("Invalid binary operator"),
                rhs: Box::new(parse_operand(p)?),
                trace: p.trace_from(start),
            }
        }

        Ok(lhs)
    }
}
