use either::Either;
use serde::{Deserialize, Serialize};
use crate::file::identifier::Identifier;
use crate::parser::ast::data::UnvalidatedType;
use crate::ir::visibility::Visibility;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnvalidatedStructPrototype {
    pub identifier: Identifier,
    pub properties: Vec<UnvalidatedStructProperty>,
    pub visibility: Visibility,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnvalidatedStructProperty {
    pub property: UnvalidatedProperty,
    pub visibility: Visibility,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnvalidatedProperty {
    pub name: Identifier,
    pub ty: UnvalidatedType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnvalidatedEnumVariant {
    pub name: Identifier,
    pub data: Option<UnvalidatedEnumData>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum UnvalidatedEnumData {
    Positional(Vec<UnvalidatedType>),
    StructLike(Vec<UnvalidatedProperty>),
}

