use serde::{Deserialize, Serialize};
use crate::file::identifier::Identifier;
use crate::parser::ast::data::UnvalidatedType;
use crate::ir::visibility::Visibility;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnvalidatedStructPrototype {
    properties: Vec<UnvalidatedStructPrototype>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnvalidatedStructProperty {
    name: Identifier,
    ty: UnvalidatedType,
    visibility: Visibility,
}