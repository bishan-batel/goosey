use crate::file::identifier::{Namespace};
use crate::file::trace::Trace;
use crate::parser::ast::data::{UnvalidatedVariableInfo};
use crate::parser::ast::expression::UnvalidatedExpression;
use crate::parser::ast::function::{UnvalidatedFunctionPrototype, UnvalidatedFunctionExpression};
use crate::parser::ast::r#struct::{UnvalidatedEnumVariant, UnvalidatedStructPrototype};
use crate::parser::ast::UnvalidatedSymbol;

#[derive(Debug, PartialEq)]
pub enum UnvalidatedTopLevel {
    FunctionDefinition {
        proto: UnvalidatedFunctionPrototype,
        body: UnvalidatedFunctionExpression,
        trace: Trace,
    },
    StructDefinition {
        proto: UnvalidatedStructPrototype,
        trace: Trace,
    },
    EnumDefinition {
        variants: Vec<UnvalidatedEnumVariant>,
        trace: Trace
    },
    GlobalVariable {
        variable: UnvalidatedVariableInfo,
        initial: UnvalidatedExpression,
        trace: Trace,
    },
    Import {
        namespace: UnvalidatedSymbol,
        star: bool,
        trace: Trace,
    },
}