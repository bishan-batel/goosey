use std::rc::Rc;
use crate::file::source_file::SourceFile;
use crate::lexer::lexer;
use crate::lexer::token::Token;

#[test]
pub fn identifier() {
    let file = SourceFile::new(r#"
        me _wh_en__ the
    "#).rc();

    let tokens: Vec<Token> = crate::lexer::tokenize(file).into_iter().map(|(f, _)| f).collect();

    assert_eq!(tokens, vec![
        Token::Identifier("me".into()),
        Token::Identifier("_wh_en__".into()),
        Token::Identifier("the".into())]
    );
}

#[test]
pub fn bool() {
    let file = SourceFile::new(r#"
        true false false true true false true
    "#).rc();

    let tokens: Vec<Token> = crate::lexer::tokenize(file).into_iter().map(|(f, _)| f).collect();

    assert_eq!(tokens, vec![
        Token::BoolLiteral(true),
        Token::BoolLiteral(false),
        Token::BoolLiteral(false),
        Token::BoolLiteral(true),
        Token::BoolLiteral(true),
        Token::BoolLiteral(false),
        Token::BoolLiteral(true),
    ]);
}

#[test]
pub fn test_operator() {
    use super::token::Operator as E;

    let file = SourceFile::new(r#"
    {}()[]~!%^&*-+/| = < >
    : && || << >> :: %= ^= &= *=
    -= += /= <<= >>= &&= ||= <= >= != == . ,
    "#).rc();

    let tokens: Vec<Token> = crate::lexer::tokenize(file).into_iter().map(|(f, _)| f).collect();
    assert_eq!(tokens, [
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
    ].map(|o| Token::Operator(o)).into_iter().collect::<Vec<Token>>());
}

#[test]
fn string() {
    let file = SourceFile::new(r#"
        "me when the \" ""yuh"
    "#).rc();

    let tokens: Vec<Token> = crate::lexer::tokenize(file).into_iter().map(|(f, _)| f).collect();

    assert_eq!(tokens, vec![
        Token::StringLiteral("me when the \" ".into()),
        Token::StringLiteral("yuh".into()),
    ]);
}
