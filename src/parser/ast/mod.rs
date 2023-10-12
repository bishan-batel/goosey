use std::ops::Range;
use serde::{Deserialize, Serialize};
use crate::file::identifier::{Identifier, Namespace};

pub mod function;
pub mod data;
pub mod expression;
pub mod top_level;
pub mod r#struct;
pub mod operations;


#[derive(Serialize, Deserialize, Hash, PartialEq, Debug)]
pub struct ParserTrace {
    token_range: Range<usize>,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Debug)]
pub struct UnvalidatedSymbol {
    pub explicit_namespace: Namespace,
    pub identifier: Identifier,
}