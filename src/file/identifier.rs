use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use std::ops::Deref;


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GlobalIdentifier(pub Namespace, pub Identifier);

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct Namespace {
    pub chain: Vec<Identifier>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct Identifier(pub String);


impl Display for GlobalIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", &self.0, &self.1))
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.chain.len() == 0 {
            f.write_str("<global>")?
        } else {
            for ident in &self.chain[0..(self.chain.len() - 1)] {
                f.write_fmt(format_args!("{ident}."))?;
            }
            f.write_fmt(format_args!("{}", self.chain.last().unwrap()))?;
        }
        Ok(())
    }
}

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier(value.into())
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}