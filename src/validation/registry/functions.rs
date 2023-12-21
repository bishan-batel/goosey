use std::hash::{Hash, Hasher};
use crate::file::identifier::GlobalIdentifier;
use crate::validation::data_type::DataType;
use crate::validation::registry::expression::FunctionStatement;
use crate::validation::registry::variable::VariableInfo;

#[derive(Debug, Eq, Clone)]
pub struct FunctionPrototype {
    pub name: GlobalIdentifier,
    pub arguments: Vec<VariableInfo>,
    pub returns: Option<DataType>,
}

impl PartialEq for FunctionPrototype {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name { return false; }
        if self.arguments.len() != other.arguments.len() { return false; }

        for (s, o) in self.arguments
            .iter()
            .zip(other.arguments.iter()) {
            if s.data_type != o.data_type {
                return false;
            }
        }

        true
    }
}

impl Hash for FunctionPrototype {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);

        for arg in self.arguments.iter() {
            arg.data_type.hash(state);
        }
    }
}