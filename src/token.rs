#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Integer(i64),
    Assign,
    Semicolon,
    LParen,
    RParen,
    Let,
    Print,
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    EOF,
}
