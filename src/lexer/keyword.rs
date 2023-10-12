use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Hash)]
pub enum Keyword {
    Let,
    Mut,
    Fun,
    Return,
    Ref,
    Deref,
    For,
    While,
    Unless,
    Else,
    Struct,
    Impl,
    Sizeof,
    As,
    Public,
    Until,
    If,
    Unit,
}

impl Keyword {
    pub fn parse(txt: &str) -> Option<Keyword> {
        use Keyword as E;
        Some(match txt {
            "let" => E::Let,
            "mut" => E::Mut,
            "fun" => E::Fun,
            "return" => E::Return,
            "ref" => E::Ref,
            "deref" => E::Deref,
            "for" => E::For,
            "while" => E::While,
            "unless" => E::Unless,
            "until" => E::Until,
            "if" => E::If,
            "else" => E::Else,
            "struct" => E::Struct,
            "impl" => E::Impl,
            "sizeof" => E::Sizeof,
            "as" => E::As,
            "pub" => E::Public,
            "unit" => E::Unit,
            _ => return None
        })
    }
}