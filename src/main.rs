use crate::lexer::token::Operator::ParenOpen;

mod lexer;
mod file;
mod parser;
mod ir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
