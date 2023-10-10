use serde::{Deserialize, Serialize};
use crate::file::Identifier;
use crate::parser::ast::data::{UnvalidatedType, UnparsedVariableInfo};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::ir::visibility::Visibility;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnparsedFunctionPrototype {
    pub name: Identifier,
    pub arguments: Vec<UnparsedVariableInfo>,
    pub returns: Option<UnvalidatedType>,
    pub visibility: Visibility,
}

/// Expressions that can not be evaluated to a value
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum UnvalidatedFunctionExpression {
    // an single if can not evaluated to an expression
    If {
        condition: UnvalidatedExpression,
        then: Box<UnvalidatedFunctionExpression>,
    },
    While {
        condition: UnvalidatedExpression,
        then: Box<UnvalidatedFunctionExpression>,
    },
    Let {
        variable: UnparsedVariableInfo,
        initial: Option<UnvalidatedExpression>,
    },
    Expression(UnvalidatedExpression),
}