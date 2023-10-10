use std::ops::Range;
use serde::{Deserialize, Serialize};

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