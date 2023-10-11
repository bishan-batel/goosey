use serde::{Deserialize, Serialize};
use crate::file::identifier::{Identifier, Namespace};
use crate::file::trace::Trace;
use crate::parser::ast::data::{UnvalidatedType, UnparsedVariableInfo};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::parser::ast::function::{UnparsedFunctionPrototype, UnvalidatedFunctionExpression};
use crate::parser::ast::r#struct::{UnvalidatedStructProperty, UnvalidatedStructPrototype};

#[derive(Debug, PartialEq)]
pub enum UnparsedTopLevel {
    FunctionDefinition {
        proto: UnparsedFunctionPrototype,
        body: UnvalidatedFunctionExpression,
        trace: Trace,
    },
    StructDefinition {
        proto: UnvalidatedStructPrototype,
        properties: Vec<UnvalidatedStructProperty>,
        trace: Trace,
    },
    GlobalVariable {
        variable: UnparsedVariableInfo,
        initial: UnvalidatedExpression,
        trace: Trace,
    },
    Import {
        namespace: Namespace,
        star: bool,
        trace: Trace,
    },
}