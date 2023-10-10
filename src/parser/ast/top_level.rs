use serde::{Deserialize, Serialize};
use crate::file::Identifier;
use crate::parser::ast::data::{UnparsedType, UnparsedVariableInfo};
use crate::parser::ast::expression::Expression;
use crate::parser::ast::function::{UnparsedFunctionPrototype, UnparsedFunctionStatement};

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum UnparsedTopLevel {
    FunctionDefinition {
        proto: UnparsedFunctionPrototype,
        body: UnparsedFunctionStatement,
    },
    StructDefinition {
        name: Identifier
    },
    GlobalVariable {
        variable: UnparsedVariableInfo,
        initial: Expression,
    },
    Import {
        path: Vec<Identifier>,
        star: bool,
    },
}