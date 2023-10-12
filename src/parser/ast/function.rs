use serde::{Deserialize, Serialize};
use crate::file::identifier::Identifier;
use crate::file::trace::Trace;
use crate::parser::ast::data::{UnvalidatedType, UnparsedVariableInfo};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::ir::visibility::Visibility;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnparsedFunctionPrototype {
    pub name: Identifier,
    pub arguments: Vec<UnparsedVariableInfo>,
    pub returns: UnvalidatedType,
    pub visibility: Visibility,
}

/// Expressions that can not be evaluated to a value
#[derive(Debug, PartialEq)]
pub enum UnvalidatedFunctionExpression {
    // an single if can not evaluated to an expression
    If {
        condition: UnvalidatedExpression,
        then: Box<UnvalidatedFunctionExpression>,
        trace: Trace,
    },
    While {
        condition: UnvalidatedExpression,
        then: Box<UnvalidatedFunctionExpression>,
        trace: Trace,
    },
    Let {
        variable: UnparsedVariableInfo,
        initial: Option<UnvalidatedExpression>,
        trace: Trace,
    },
    Return(Option<UnvalidatedExpression>),
    Expression(UnvalidatedExpression),
}