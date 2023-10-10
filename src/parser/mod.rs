use crate::file::trace::Trace;
use crate::lexer::token::TokenData;

pub mod ast;
mod parser_modules;


pub struct Parser {
    source: Vec<(TokenData, Trace)>,
}