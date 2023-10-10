use std::rc::Rc;
use crate::file::source_file::SourceFile;
use crate::file::trace::Trace;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenData};

mod lexer;
pub mod token;
pub mod keyword;

#[cfg(test)]
mod test;

pub fn tokenize(file: Rc<SourceFile>) -> Vec<Token> {
    Lexer::new(file).tokenize()
}