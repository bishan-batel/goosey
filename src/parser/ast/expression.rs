use serde::{Deserialize, Serialize};
use crate::parser::ast::data::UnparsedVariableInfo;
use crate::parser::ast::function::UnvalidatedFunctionExpression;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum UnvalidatedExpression {
    // if else chains can evaluated to an expression
    IfElse {
        condition: UnvalidatedExpression,
        then: Box<UnvalidatedFunctionExpression>,
        otherwise: Box<UnvalidatedFunctionExpression>,
    },
    Scope(Vec<UnvalidatedFunctionExpression>),
    BinaryExpression,
}