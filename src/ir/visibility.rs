use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum Visibility {
    Public,
    Private,
}