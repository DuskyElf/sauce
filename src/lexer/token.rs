use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")] 
pub enum Token {
    #[token("grab")]
    Grab,
    #[token("yell")]
    Yell,
    #[token("toss")]
    Toss,
    #[token("|>")]
    Pipe,

    #[token("=")]
    Equals, 

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex(r"[0-9]+")]
    Int,
    
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    String,

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

    #[regex(r".", priority = 0)] 
    Error,
}