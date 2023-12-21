use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use crate::file::identifier::Identifier;
use crate::validation::error::CompilerResult;
use crate::validation::registry::ModuleRegistry;
use crate::validation::registry::variable::VariableInfo;

pub type LocalVariableTable = HashMap<Uuid, VariableInfo>;

#[derive(Debug)]
pub struct FunctionContext {
    pub global_registry: Rc<ModuleRegistry>,
    local_map: LocalVariableTable,
    scopes: Vec<Vec<(Identifier, Uuid)>>,
}

impl FunctionContext {
    pub fn new(registry: Rc<ModuleRegistry>) -> Self {
        Self {
            global_registry: registry,
            local_map: Default::default(),
            scopes: vec![],
        }
    }

    pub fn declare_variable(&mut self, local: VariableInfo) -> CompilerResult<Uuid> {

        // generate a new UUID for hygienic name creation
        let uuid = Uuid::new_v4();

        self.scopes
            .last_mut()
            .expect("Null scope")
            .push((local.name.clone(), uuid));

        self.local_map.insert(uuid, local);
        Ok(uuid)
    }

    pub fn get_local(&self, name: &Identifier) -> Option<(Uuid, &VariableInfo)> {

        // for each scope back to front
        for scope in self.scopes.iter().rev() {

            // search scope for variable name prioritizing more recent names
            for (local, uuid) in scope.iter().rev() {
                if local == name {
                    let info = self.local_map.get(uuid).expect("");
                    return Some((*uuid, info));
                }
            }
        }
        None
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Default::default());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop().expect("Empty function context stack");
    }

    pub fn get_locals_table(&self) -> &LocalVariableTable {
        &self.local_map
    }
}