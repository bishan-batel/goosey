use std::rc::Rc;
use crate::file::source_file::SourceFile;
use crate::file::trace::Trace;
use crate::lexer::keyword::Keyword;
use crate::lexer::token::Token;

#[derive(Debug)]
pub struct Lexer {
    file: Rc<SourceFile>,
    index: usize,
}

impl Lexer {
    pub fn new(source: Rc<SourceFile>) -> Self {
        Self {
            file: source,
            index: 0,
        }
    }

    pub fn curr(&self) -> char {
        self.file.source.chars().nth(self.index).unwrap_or(' ')
    }

    pub fn trace_from(&self, from: usize, offset: usize) -> Trace {
        self.file.trace(from..(from + offset))
    }

    pub fn trace(&self, offset: usize) -> Trace {
        self.file.trace(self.index..offset)
    }

    pub fn advance(&mut self) -> char {
        self.advance_by(1)
    }

    pub fn advance_by(&mut self, delta: usize) -> char {
        self.index += delta;
        self.curr()
    }

    pub fn slice(&mut self, offset: usize) -> &str {
        let upper = (self.index + offset).min(self.file.source.len());
        &self.file.source[self.index..upper]
    }

    #[inline]
    pub fn not_eof(&mut self) -> bool {
        self.has_clearance(1)
    }

    pub fn has_clearance(&self, i: usize) -> bool {
        self.index + i <= self.file.source.len()
    }

    pub fn tokenize(mut self) -> Vec<(Token, Trace)> {
        let passes = &[
            Self::identifier,
            Self::string,
            Self::number,
            Self::tri_operator,
            Self::dual_operator,
            Self::operator
        ];

        let mut toks = vec![];

        while self.not_eof() {
            if self.curr().is_whitespace() {
                self.advance();
                continue;
            }

            if self.comment() {
                continue;
            }

            for pass in passes {
                if let Some(tok) = pass(&mut self) {
                    toks.push(tok);
                    break;
                }
            }
        }

        toks
    }

    pub fn comment(&mut self) -> bool {
        if !self.has_clearance(2) {
            return false;
        }

        match self.slice(2) {
            "//" => {
                while self.not_eof() && self.advance() != '\n' {}
                true
            }
            "/*" => {
                while self.not_eof() && self.slice(2) != "*/" {}
                true
            }
            _ => false
        }
    }

    pub fn identifier(&mut self) -> Option<(Token, Trace)> {
        if !self.curr().is_ascii_alphabetic() && self.curr() != '_' {
            return None;
        }

        let start = self.index;

        while self.advance().is_ascii_alphanumeric() || self.curr() == '_' {}

        if self.curr() == '!' {
            self.advance();
            let slice = &self.file.source[start..self.index];
            let tok = Token::MacroIdentifier(String::from(slice));

            return Some((tok, self.trace_from(start, slice.len())));
        }

        let ident = &self.file.source[start..self.index];

        if let Some(keyword) = Keyword::parse(ident) {
            Some((Token::Keyword(keyword), self.trace_from(start, ident.len())))
        } else {
            Some(match ident {
                "true" => (Token::BoolLiteral(true), self.trace_from(start, "true".len())),
                "false" => (Token::BoolLiteral(false), self.trace_from(start, "false".len())),
                _ => (Token::Identifier(ident.into()), self.trace_from(start, ident.len()))
            })
        }
    }

    pub fn string(&mut self) -> Option<(Token, Trace)> {
        if self.curr() != '"' {
            return None;
        }

        let start = self.index;

        let mut builder = string_builder::Builder::default();
        while self.not_eof() && self.advance() != '"' {
            if self.curr() == '\\' {
                builder.append(self.advance());
            } else {
                builder.append(self.curr());
            }
        }
        self.advance();

        let tok = Token::StringLiteral(builder.string().expect("UTF8 Error"));
        Some((tok, self.trace_from(start, self.index - 1)))
    }

    pub fn number(&mut self) -> Option<(Token, Trace)> {
        if !(self.curr().is_ascii_digit() || self.curr() != '.') {
            return None;
        }

        None
    }

    pub fn operator(&mut self) -> Option<(Token, Trace)> {
        use super::token::Operator as E;

        let op = Token::Operator(match self.curr() {
            '{' => E::CurlyOpen,
            '}' => E::CurlyClose,
            '(' => E::ParenOpen,
            ')' => E::ParenClose,
            '[' => E::BracketOpen,
            ']' => E::BracketClose,
            '!' => E::Not,
            '%' => E::Mod,
            '^' => E::Xor,
            '&' => E::BitAnd,
            '*' => E::Multiply,
            '-' => E::Minus,
            '+' => E::Plus,
            '/' => E::Divide,
            '|' => E::BitOr,
            '=' => E::Assign,
            '<' => E::Less,
            '>' => E::Greater,
            '~' => E::BitNot,
            '.' => E::Dot,
            ',' => E::Comma,
            ':' => E::Colon,
            _ => return None
        });
        self.advance();
        Some((op, self.trace(1)))
    }

    pub fn dual_operator(&mut self) -> Option<(Token, Trace)> {
        if !self.has_clearance(2) {
            return None;
        }

        use super::token::Operator as E;
        let op = Some((Token::Operator(match self.slice(2) {
            "&&" => E::And,
            "||" => E::Or,
            "<<" => E::BitShiftLeft,
            ">>" => E::BitShiftRight,
            "%=" => E::ModAssigns,
            "^=" => E::XorAssign,
            "&=" => E::BitAndAssign,
            "*=" => E::MultiplyAssign,
            "-=" => E::MinusAssign,
            "+=" => E::PlusAssign,
            "/=" => E::DivideAssign,
            "<=" => E::LessEquals,
            ">=" => E::GreaterEquals,
            "!=" => E::NotEquals,
            "==" => E::Equals,
            "::" => E::DoubleColon,

            _ => return None
        }), self.trace(2)));
        self.advance_by(2);
        op
    }

    pub fn tri_operator(&mut self) -> Option<(Token, Trace)> {
        if !self.has_clearance(3) {
            return None;
        }

        use super::token::Operator as E;
        let op = Some((Token::Operator(match self.slice(3) {
            "<<=" => E::BitShiftLeftAssign,
            ">>=" => E::BitShiftRightAssign,
            "&&=" => E::AndAssign,
            "||=" => E::OrAssign,
            _ => return None
        }), self.trace(3)));
        self.advance_by(3);

        op
    }
}