use serde::{Deserialize, Serialize};
use crate::lexer::token::Operator;

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

impl TryFrom<Operator> for BinaryOperation {
    type Error = ();

    fn try_from(op: Operator) -> Result<Self, Self::Error> {
        use Operator as E;
        use BinaryOperation as B;
        Ok(match op {
            E::Mod => B::Mod,
            E::Xor => B::Xor,
            E::BitAnd => B::BitAnd,
            E::Multiply => B::Multiply,
            E::Minus => B::Minus,
            E::Plus => B::Plus,
            E::Divide => B::Divide,
            E::BitOr => B::BitOr,
            E::Assign => B::Assign,
            E::Less => B::Less,
            E::Greater => B::Greater,
            E::Colon => B::Colon,

            E::ModAssigns => B::ModAssigns,
            E::XorAssign => B::XorAssign,
            E::BitAndAssign => B::BitAndAssign,
            E::MultiplyAssign => B::MultiplyAssign,
            E::MinusAssign => B::MinusAssign,
            E::PlusAssign => B::PlusAssign,
            E::DivideAssign => B::DivideAssign,

            E::BitShiftLeftAssign => B::BitShiftLeftAssign,
            E::BitShiftRightAssign => B::BitShiftRightAssign,
            E::AndAssign => B::AndAssign,
            E::OrAssign => B::OrAssign,

            E::Equals => B::Equals,
            E::LessEquals => B::LesserOrEquals,
            E::GreaterEquals => B::GreaterOrEquals,
            E::NotEquals => B::NotEqual,
            _ => return Err(())
        })
    }
}