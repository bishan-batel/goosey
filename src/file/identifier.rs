use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use std::ops::Deref;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Hash)]
pub struct GlobalIdentifier(pub Namespace, pub Identifier);

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct Namespace {
    pub chain: Vec<Identifier>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct Identifier(pub String);

impl GlobalIdentifier {
    pub fn prefix(&self, namespace: Namespace) -> Self {
        Self(self.0.prefix(namespace), self.1.clone())
    }
}

impl Namespace {
    pub fn global() -> Self {
        Self {
            chain: vec!["main".into()],
        }
    }

    pub fn prefix(&self, prefix: Namespace) -> Self {
        Self {
            chain: prefix.chain
                .into_iter()
                .chain(self.chain.clone().into_iter())
                .collect(),
        }
    }

    pub fn module(&self, ident: Identifier) -> Self {
        let mut namespace = self.clone();
        namespace.chain.push(ident);
        namespace
    }
}


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