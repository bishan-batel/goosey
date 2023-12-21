use crate::file::identifier::Identifier;
use crate::validation::data_type::DataType;


// #[derive(Debug, Hash, Eq, PartialEq, Clone)]
// pub struct LocalVariable {
//     pub info: VariableInfo,
//     pub mutable: bool,
// }
//
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct VariableInfo {
    pub name: Identifier,
    pub data_type: DataType,
    pub mutable: bool
}

// impl From<DataType> for VariableInfo {
//     fn from(value: DataType) -> Self {
//         Self {
//             name: "_".into(),
//             data_type: value,
//         }
//     }
// }