use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use serde::{Deserialize, Serialize};

pub mod source_file;
pub mod trace;

pub struct GlobalIdentifier(Vec<Identifier>);

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct Identifier(pub String);

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier(value.into())
    }
}
