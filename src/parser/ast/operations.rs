use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum BinaryOperation {
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

    Equals,
    LesserOrEquals,
    GreaterOrEquals,
    NotEqual,
}

pub enum UnaryOperator {
    Ref,
    Deref,
    Negate,
    Not,
    BitNot,
}