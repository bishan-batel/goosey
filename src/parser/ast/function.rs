use serde::{Deserialize, Serialize};
use crate::file::Identifier;
use crate::parser::ast::data::{UnparsedType, UnparsedVariableInfo};
use crate::parser::ast::expression::Expression;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnparsedFunctionPrototype {
    pub name: Identifier,
    pub arguments: Vec<UnparsedVariableInfo>,
    pub returns: Option<UnparsedType>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum UnparsedFunctionStatement {
    Scope(Vec<UnparsedFunctionStatement>),
    If {
        condition: Expression,
        then: Box<UnparsedFunctionStatement>,
    },
    While {
        condition: Expression,
        then: Box<UnparsedFunctionStatement>,
    },
    Let {
        variable: UnparsedVariableInfo,
        initial: Option<Expression>,
    },
    Expression(Expression),
}