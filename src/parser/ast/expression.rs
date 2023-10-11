use serde::{Deserialize, Serialize};
use crate::file::trace::Trace;
use crate::lexer::token::Operator;
use crate::parser::ast::data::UnparsedVariableInfo;
use crate::parser::ast::function::UnvalidatedFunctionExpression;

#[derive(Debug, PartialEq)]
pub enum UnvalidatedExpression {
    // if else chains can evaluated to an expression
    IfElse {
        condition: Box<UnvalidatedExpression>,
        then: Box<UnvalidatedFunctionExpression>,
        otherwise: Box<UnvalidatedFunctionExpression>,
    },
    Scope(Vec<UnvalidatedFunctionExpression>),

    BoolLiteral(bool),
    I32Literal(f32),
    I64Literal(f64),
    StringLiteral(String),

    Parenthetical {
        expr: Box<UnvalidatedExpression>,
    },
    BinaryExpression {
        lhs: Box<UnvalidatedExpression>,
        op: Operator,
        rhs: Box<UnvalidatedExpression>,
    },
}