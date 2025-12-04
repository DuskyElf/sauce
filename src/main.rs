#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token("grab")]
    Grab,

    #[token("yell")]
    Yell,

    #[token("toss")]
    Toss,

    #[token("|>")]
    Pipe,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    #[regex(r"[0-9]+")]
    Int,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,

    #[regex(r"[ \t\r\n]+", logos::skip)]
    Whitespace,

    #[error]
    Error,
}
fn main() {
    println!("Hello, world!");
}
