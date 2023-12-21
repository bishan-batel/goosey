use std::ops::Range;
use std::rc::Rc;
use crate::file::identifier::{Identifier, Namespace};
use crate::file::source_file::SourceFile;
use crate::file::trace::Trace;
use crate::lexer::keyword::Keyword;
use crate::lexer::token::{Operator, Token, TokenData};
use crate::parser::ast::data::UnvalidatedType;
use crate::parser::ast::top_level::UnvalidatedTopLevel;
use crate::parser::ast::UnvalidatedSymbol;
use crate::parser::error::{ParserError, ParserResult};
use crate::parser::modules::top_level::TopLevelParser;

pub struct Parser {
    source: Vec<Token>,
    file: Rc<SourceFile>,
    position: usize,
}

impl Parser {
    pub fn new(file: Rc<SourceFile>, tokens: Vec<Token>) -> Self {
        Self {
            source: tokens,
            file,
            position: 0,
        }
    }

    pub fn curr(&self) -> &Token {
        &self.source[self.position.min(self.source.len())]
    }

    pub fn advance(&mut self) -> &Token {
        self.position += 1;
        self.curr()
    }

    pub fn rollback(&mut self) {
        self.position -= 1;
    }

    #[inline]
    pub fn position(&self) -> usize {
        self.position
    }

    pub fn trace(&self, range: Range<usize>) -> Trace {
        self.file.trace(range)
    }

    pub fn trace_from(&self, token_index: usize) -> Trace {
        let prev = self.source[token_index].trace().range.clone();
        let curr = self.curr().trace().range.clone();

        let min = prev.start.min(curr.start);
        let max = prev.end.max(curr.end);
        self.trace(min..max)
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

    pub fn has_identifier(&mut self) -> bool {
        match self.curr().token() {
            TokenData::Identifier(_) => true,
            _ => false
        }
    }

    pub fn consume_identifier(&mut self) -> ParserResult<Identifier> {
        let curr = self.curr().clone();

        match curr.token() {
            TokenData::Identifier(ident) => {
                self.advance();
                Ok(ident.clone())
            }
            _ => Err(ParserError::ExpectedIdentifier(curr.trace().clone()))
        }
    }

    pub fn consume_operator(&mut self) -> ParserResult<Operator> {
        let curr = self.curr().clone();
        self.advance();
        match curr.token() {
            TokenData::Operator(op) => {
                self.advance();
                Ok(*op)
            }
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
            self.advance();
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

    pub fn consume_symbol(&mut self) -> ParserResult<UnvalidatedSymbol> {
        let mut identifier = self.consume_identifier()?;

        if !self.has_operator(Operator::DoubleColon) {
            return Ok(UnvalidatedSymbol {
                explicit_namespace: Namespace { chain: vec![] },
                identifier,
            });
        }

        let mut chain = vec![identifier];

        while self.has_operator(Operator::DoubleColon) {
            self.advance();
            chain.push(self.consume_identifier()?);
        }

        Ok(UnvalidatedSymbol {
            identifier: chain.pop().expect("Unreachable"),
            explicit_namespace: Namespace {
                chain,
            },
        })
    }

    pub fn parse(mut self) -> ParserResult<Vec<UnvalidatedTopLevel>> {
        let mut statements = vec![];

        let passes = &[TopLevelParser::parse_top_level];

        'file_loop: while !self.is_eof() {
            for pass in passes {
                if let Some(statement) = pass(&mut self)? {
                    statements.push(statement);
                    continue 'file_loop;
                }
            }
            return Err(ParserError::UnexpectedToken(self.curr().clone()));
        }


        Ok(statements)
    }

    pub fn consume_type(&mut self) -> ParserResult<UnvalidatedType> {
        // reference type
        if self.has_keyword(Keyword::Ref) {
            self.advance();
            self.expect_operator(Operator::BracketOpen)?;
            let ty = self.consume_type()?;
            self.expect_operator(Operator::BracketClose)?;
            Ok(UnvalidatedType::Reference(Box::new(ty)))
        } else if self.has_operator(Operator::BracketOpen) {
            self.advance();
            let ty = self.consume_type()?;
            self.expect_operator(Operator::BracketClose)?;
            Ok(UnvalidatedType::Array(Box::new(ty)))
        } else if self.has_identifier() {
            let ty = self.consume_identifier()?;

            if self.has_operator(Operator::Less) {
                self.advance();
                let mut template_arguments = vec![];

                while !self.has_operator(Operator::Greater) && !self.is_eof() {
                    template_arguments.push(self.consume_type()?);

                    if self.has_operator(Operator::Comma) {
                        self.advance();
                    }
                    break;
                }
                self.expect_operator(Operator::Greater)?;

                Ok(UnvalidatedType::Template {
                    base: ty,
                    template_arguments,
                })
            } else {
                Ok(UnvalidatedType::Type(ty))
            }
        } else {
            Err(ParserError::UnexpectedToken(self.curr().clone()))
        }
    }
}
