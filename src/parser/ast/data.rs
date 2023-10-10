use serde::{Deserialize, Serialize};
use crate::file::Identifier;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum UnparsedType {
    Type(Identifier),
    Reference(Box<UnparsedType>),
    Template {
        owner: Box<UnparsedType>,
        template_arguments: Vec<UnparsedType>,
    },
    Implicit,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct UnparsedVariableInfo {
    pub ident: Identifier,
    pub ty: UnparsedType,
    pub mutable: bool,
}
