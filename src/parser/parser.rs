use crate::file::identifier::Identifier;
use crate::lexer::keyword::Keyword;
use crate::lexer::token::{Operator, Token, TokenData};
use crate::parser::ast::top_level::UnparsedTopLevel;
use crate::parser::error::{ParserError, ParserResult};

pub struct Parser {
    source: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            source: tokens,
            index: 0,
        }
    }

    pub fn curr(&self) -> &Token {
        &self.source[self.index.min(self.source.len())]
    }

    pub fn advance(&mut self) -> &Token {
        self.index += 1;
        self.curr()
    }

    pub fn is_eof(&self) -> bool {
        self.curr().token() == &TokenData::EOF
    }

    pub fn consume_tok(&mut self, tok: TokenData) -> ParserResult<()> {
        if self.curr().token() == &tok {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(self.curr().clone()))
        }
    }

    pub fn consume_identifier(&mut self) -> ParserResult<Identifier> {
        let curr = self.curr();
        match curr.token() {
            TokenData::Identifier(ident) => Ok(ident.clone()),
            _ => Err(ParserError::ExpectedIdentifier(curr.trace().clone()))
        }
    }

    pub fn consume_operator(&mut self) -> ParserResult<Operator> {
        let curr = self.curr();
        match curr.token() {
            TokenData::Operator(op) => Ok(*op),
            _ => Err(ParserError::ExpectedAnyOperator(curr.trace().clone()))
        }
    }

    pub fn has_operator(&self, op: Operator) -> bool {
        match self.curr().token() {
            TokenData::Operator(curr) if curr == &op => true,
            _ => false
        }
    }

    pub fn expect_operator(&mut self, op: Operator) -> ParserResult<()> {
        if self.has_operator(op.clone()) {
            Ok(())
        } else {
            Err(ParserError::ExpectedOperator(
                op,
                self.curr().trace().clone(),
            ))
        }
    }

    pub fn has_keyword(&self, word: Keyword) -> bool {
        match self.curr().token() {
            TokenData::Keyword(curr) if curr == &word => true,
            _ => false
        }
    }

    pub fn expect_keyword(&mut self, word: Keyword) -> ParserResult<()> {
        if self.has_keyword(word.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::ExpectedKeyword(
                word,
                self.curr().trace().clone(),
            ))
        }
    }

    pub fn parse(mut self) -> ParserResult<Vec<UnparsedTopLevel>> {
        let levels = vec![];

        Ok(levels)
    }
}
