use std::rc::Rc;
use crate::file::source_file::SourceFile;
use crate::parser::ast::top_level::UnvalidatedTopLevel;
use crate::validation::error::CompilerResult;
use crate::validation::registry::ModuleRegistry;

#[derive(Debug)]
pub struct Module {
    global_registry: ModuleRegistry,
}

impl Module {
    pub fn new() -> Self {
        Self {
            global_registry: ModuleRegistry::create_global()
        }
    }

    pub fn process_modules(&mut self, modules: &[(Rc<SourceFile>, Vec<UnvalidatedTopLevel>)]) -> CompilerResult<()> {
        for (file, statements) in modules {
            self.global_registry.register_sub_module(file.module_name.clone())?;
        }

        Ok(())
    }
}
