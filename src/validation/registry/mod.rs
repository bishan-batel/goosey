use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use crate::file::identifier::{GlobalIdentifier, Identifier, Namespace};
use crate::validation::data_type::DataType;
use crate::validation::error::{CompilerError, CompilerResult};
use crate::validation::registry::functions::FunctionPrototype;

pub mod enums;
pub mod functions;
pub mod structs;
pub mod variable;
mod expression;

#[derive(Debug)]
pub struct ModuleRegistry {
    path: Namespace,
    sub_modules: HashMap<Identifier, ModuleRegistry>,
    data_types: HashMap<Identifier, DataType>,
    functions: HashSet<FunctionPrototype>,
}

impl ModuleRegistry {
    pub fn create_global() -> Self {
        Self {
            path: Namespace::global(),
            sub_modules: Default::default(),
            data_types: Default::default(),
            functions: Default::default(),
        }
    }

    pub fn register_sub_module(&mut self, name: Identifier) -> CompilerResult<()> {
        let registry = Self {
            path: self.path.module(name.clone()),
            sub_modules: Default::default(),
            data_types: Default::default(),
            functions: Default::default(),
        };
        if self.sub_modules.contains_key(&name) {
            Err(CompilerError::DuplicateNamespace(self.path.module(name)))
        } else {
            self.sub_modules.insert(name, registry);
            Ok(())
        }
    }


    pub fn register_function_type(&mut self, name: GlobalIdentifier) {}

    pub fn get_function_by_identifier(
        &self,
        name: Identifier,
        arguments: Vec<DataType>,
    ) -> CompilerResult<Option<&FunctionPrototype>> {
        Ok(None)
    }

    pub fn search_for(&self, search: FunctionPrototype) {}

    pub fn create_identifier(&self, name: Identifier) -> GlobalIdentifier {
        GlobalIdentifier(self.path.clone(), name)
    }
}

impl PartialEq for ModuleRegistry {
    fn eq(&self, other: &Self) -> bool {
        other.path == self.path
    }
}

impl Hash for ModuleRegistry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
