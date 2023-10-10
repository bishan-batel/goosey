use serde::{Deserialize, Serialize};

pub mod source_file;
pub mod trace;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct Identifier(pub String);