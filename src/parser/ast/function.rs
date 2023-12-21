use serde::{Deserialize, Serialize};
use crate::file::identifier::Identifier;
use crate::file::trace::Trace;
use crate::parser::ast::data::{UnvalidatedType, UnvalidatedVariableInfo};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::ir::visibility::Visibility;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnvalidatedFunctionPrototype {
    pub name: Identifier,
    pub arguments: Vec<UnvalidatedVariableInfo>,
    pub returns: UnvalidatedType,
    pub visibility: Visibility,
}

/// Expressions that can not be evaluated to a value
#[derive(Debug, PartialEq)]
pub enum UnvalidatedFunctionExpression {
    // an single if can not evaluated to an expression
    If {
        condition: UnvalidatedExpression,
        then: Box<UnvalidatedExpression>,
        trace: Trace,
    },
    While {
        condition: UnvalidatedExpression,
        then: Box<UnvalidatedExpression>,
        trace: Trace,
    },
    Let {
        variable: UnvalidatedVariableInfo,
        initial: Option<UnvalidatedExpression>,
        trace: Trace,
    },
    Return(Option<UnvalidatedExpression>),
    Expression(UnvalidatedExpression),
}