use serde::{Deserialize, Serialize};
use crate::lexer::keyword::Keyword;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Token {
    MacroIdentifier(String),
    Identifier(String),
    Operator(Operator),
    Keyword(Keyword),
    StringLiteral(String),
    BoolLiteral(bool),
    F32Literal(f32),
    F64Literal(f64),
    I32Literal(i32),
    I64Literal(f64),
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