use crate::file::identifier::Identifier;
use crate::file::source_file::SourceFile;

use crate::lexer::token::{Operator, Token, TokenData};

#[test]
pub fn identifier() {
    let file = SourceFile::new(r#"
        me _wh_en__ the
    "#).rc();

    let tokens: Vec<TokenData> = crate::lexer::tokenize(file).into_iter().map(|Token(f, _)| f).collect();

    assert_eq!(tokens, vec![
        TokenData::Identifier(Identifier::from("me")),
        TokenData::Identifier(Identifier::from("_wh_en__")),
        TokenData::Identifier(Identifier::from("the")),
        TokenData::EOF,
    ]);
}

#[test]
pub fn bool() {
    let file = SourceFile::new(r#"
        true false false true true false true
    "#).rc();

    let tokens: Vec<TokenData> = crate::lexer::tokenize(file).into_iter().map(|Token(f, _)| f).collect();

    assert_eq!(tokens, vec![
        TokenData::BoolLiteral(true),
        TokenData::BoolLiteral(false),
        TokenData::BoolLiteral(false),
        TokenData::BoolLiteral(true),
        TokenData::BoolLiteral(true),
        TokenData::BoolLiteral(false),
        TokenData::BoolLiteral(true)
        , TokenData::EOF,
    ]);
}

#[test]
pub fn test_operator() {
    use super::token::Operator as E;

    let file = SourceFile::new(r#"
    {}()[]~!%^&*-+/| = < >
    : && || << >> :: %= ^= &= *=
    -= += /= <<= >>= &&= ||= <= >= != == . ,
    => ->
    "#).rc();

    let tokens: Vec<TokenData> = crate::lexer::tokenize(file).into_iter().map(|Token(f, _)| f).collect();
    let mut expect = [
        E::CurlyOpen,
        E::CurlyClose,
        E::ParenOpen,
        E::ParenClose,
        E::BracketOpen,
        E::BracketClose,
        E::BitNot,
        E::Not,
        E::Mod,
        E::Xor,
        E::BitAnd,
        E::Multiply,
        E::Minus,
        E::Plus,
        E::Divide,
        E::BitOr,
        E::Assign,
        E::Less,
        E::Greater,
        E::Colon,
        E::And,
        E::Or,
        E::BitShiftLeft,
        E::BitShiftRight,
        E::DoubleColon,
        E::ModAssigns,
        E::XorAssign,
        E::BitAndAssign,
        E::MultiplyAssign,
        E::MinusAssign,
        E::PlusAssign,
        E::DivideAssign,
        E::BitShiftLeftAssign,
        E::BitShiftRightAssign,
        E::AndAssign,
        E::OrAssign,
        E::LessEquals,
        E::GreaterEquals,
        E::NotEquals,
        E::Equals,
        E::Dot,
        E::Comma,
        E::ThickRightArrow,
        E::ThinRightArrow,
    ].map(|o| TokenData::Operator(o)).into_iter().collect::<Vec<TokenData>>();
    expect.push(TokenData::EOF);
    assert_eq!(tokens, expect)
}

#[test]
fn string() {
    let file = SourceFile::new(r#"
        "me when the \" ""yuh"
    "#).rc();
    let tokens: Vec<TokenData> = crate::lexer::tokenize(file).into_iter().map(|Token(f, _)| f).collect();

    assert_eq!(tokens, vec![
        TokenData::StringLiteral("me when the \" ".into()),
        TokenData::StringLiteral("yuh".into()),
        TokenData::EOF,
    ]);
}

#[test]
fn number() {
    let file = SourceFile::new(r#"
        1 4 2.0. 9L 2.f 10.0d 0.1f
    "#).rc();
    let tokens: Vec<TokenData> = crate::lexer::tokenize(file).into_iter().map(|Token(f, _)| f).collect();

    assert_eq!(tokens, vec![
        TokenData::I32Literal(1),
        TokenData::I32Literal(4),
        TokenData::F32Literal(2.0),
        TokenData::Operator(Operator::Dot),
        TokenData::I64Literal(9),
        TokenData::F32Literal(2.),
        TokenData::F64Literal(10.),
        TokenData::F32Literal(0.1f32),
        TokenData::EOF,
    ]);
}

#[test]
fn test_macro() {
    let file = SourceFile::new(r#"
        bruh moment.println!("huh")
    "#).rc();
    let tokens: Vec<TokenData> = crate::lexer::tokenize(file).into_iter().map(|Token(f, _)| f).collect();

    assert_eq!(tokens, vec![
        TokenData::Identifier("bruh".into()),
        TokenData::Identifier("moment".into()),
        TokenData::Operator(Operator::Dot),
        TokenData::MacroIdentifier("println!".into()),
        TokenData::Operator(Operator::ParenOpen),
        TokenData::StringLiteral("huh".into()),
        TokenData::Operator(Operator::ParenClose),
        TokenData::EOF,
    ]);
}