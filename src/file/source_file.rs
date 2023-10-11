use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use crate::file::identifier::Identifier;
use crate::file::trace::Trace;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SourceFile {
    pub module_name: Identifier,
    pub source: String,
}

impl SourceFile {
    pub fn new(source: &str) -> Self {
        Self {
            module_name: "<main>".into(),
            source: String::from(source),
        }
    }

    pub fn create_from_file(path: &str) -> Result<Self, std::io::Error> {
        let mut contents = String::new();

        let path = Path::new(path);

        let mut file = File::open(path)?;
        file.read_to_string(&mut contents)?;

        let module_name: String = path.file_name().unwrap().to_str().unwrap().into();
        //
        let module_name: String = module_name.split(".").next().unwrap().into();

        Ok(Self {
            module_name: Identifier(module_name),
            source: contents,
        })
    }

    pub fn trace(self: &Rc<Self>, range: Range<usize>) -> Trace {
        Trace {
            source: Rc::clone(self),
            range,
        }
    }

    pub fn rc(self) -> Rc<Self> {
        Rc::new(self)
    }
}