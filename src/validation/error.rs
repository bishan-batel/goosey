use crate::file::identifier::Namespace;
use crate::file::trace::Trace;

pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug)]
pub enum CompilerError {
    UnknownNamespace(Namespace, Trace),
    DuplicateNamespace(Namespace),
}

impl CompilerError {
    pub fn trace(&self) -> Option<&Trace> {
        match self {
            CompilerError::UnknownNamespace(_, trace) => Some(trace),
            CompilerError::DuplicateNamespace(_) => None,
        }
    }
}
