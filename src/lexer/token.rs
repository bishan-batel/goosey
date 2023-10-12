use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use crate::file::identifier::Identifier;
use crate::file::trace::Trace;
use crate::lexer::keyword::Keyword;

#[derive(Debug, PartialEq, Clone)]
pub struct Token(pub TokenData, pub Trace);

impl Token {
    #[inline]
    pub const fn token(&self) -> &TokenData {
        &self.0
    }

    #[inline]
    pub const fn trace(&self) -> &Trace {
        &self.1
    }
}


impl Deref for Token {
    type Target = TokenData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Token {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TokenData {
    MacroIdentifier(Identifier),
    Identifier(Identifier),
    Operator(Operator),
    Keyword(Keyword),
    StringLiteral(String),
    BoolLiteral(bool),
    F32Literal(f32),
    F64Literal(f64),
    I32Literal(i32),
    I64Literal(i64),
    EOF,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Hash)]
pub enum Operator {
    CurlyOpen,
    CurlyClose,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,

    BitNot,
    Not,
    Mod,
    Xor,
    BitAnd,
    Multiply,
    Minus,
    Plus,
    Divide,
    BitOr,
    Assign,
    Less,
    Greater,
    Colon,

    And,
    Or,
    BitShiftLeft,
    BitShiftRight,
    DoubleColon,
    ThickRightArrow,
    ThinRightArrow,

    ModAssigns,
    XorAssign,
    BitAndAssign,
    MultiplyAssign,
    MinusAssign,
    PlusAssign,
    DivideAssign,

    BitShiftLeftAssign,
    BitShiftRightAssign,
    AndAssign,
    OrAssign,

    LessEquals,
    GreaterEquals,
    NotEquals,
    Equals,

    Dot,
    Comma,
}