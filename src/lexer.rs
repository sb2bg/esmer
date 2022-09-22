use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("^")]
    Caret,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[regex(r"\d+(\.\d+)?", |lexer| lexer.slice().parse::<f64>())]
    Number(f64),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lexer| lexer.slice().to_string())]
    Identifier(String),

    #[error]
    #[regex(r"[ \t]+", logos::skip)] // skip whitespace
    Error,
}
