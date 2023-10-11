use serde::{Deserialize, Serialize};
use crate::file::Identifier;
use crate::parser::ast::data::{UnvalidatedType, UnparsedVariableInfo};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::parser::ast::function::{UnparsedFunctionPrototype, UnvalidatedFunctionExpression};
use crate::parser::ast::r#struct::{UnvalidatedStructProperty, UnvalidatedStructPrototype};

#[derive(Debug, PartialEq)]
pub enum UnparsedTopLevel {
    FunctionDefinition {
        proto: UnparsedFunctionPrototype,
        body: UnvalidatedFunctionExpression,
    },
    StructDefinition {
        proto: UnvalidatedStructPrototype
    },
    GlobalVariable {
        variable: UnparsedVariableInfo,
        initial: UnvalidatedExpression,
    },
    Import {
        path: Vec<Identifier>,
        star: bool,
    },
}