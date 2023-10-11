use serde::{Deserialize, Serialize};
use crate::file::identifier::Identifier;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum UnvalidatedType {
    Type(Identifier),
    Reference(Box<UnvalidatedType>),
    Template {
        owner: Box<UnvalidatedType>,
        template_arguments: Vec<UnvalidatedType>,
    },
    Implicit,
    Unit,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnparsedVariableInfo {
    pub ident: Identifier,
    pub ty: UnvalidatedType,
    pub mutable: bool,
}
