use sauce::lexer::{Lexer, Token};
use sauce::util::span::Span;
use sauce::errors::lex::LexError;

#[test]
fn test_basic_pipeline() {
    let src = "grab x = 10 |> yell";
    let lexer = Lexer::new(src);
    let tokens: Vec<_> = lexer.collect();

    assert_eq!(tokens.len(), 6);

    let t0 = tokens[0].as_ref().expect("valid");
    assert_eq!(t0.token, Token::Grab);
    assert_eq!(t0.span, Span::new(0, 4));

    let t1 = tokens[1].as_ref().expect("valid");
    assert_eq!(t1.token, Token::Ident);
    assert_eq!(t1.span, Span::new(5, 6));

    let t2 = tokens[2].as_ref().expect("valid");
    assert_eq!(t2.token, Token::Equals); 

    let t3 = tokens[3].as_ref().expect("valid");
    assert_eq!(t3.token, Token::Int);

    let t4 = tokens[4].as_ref().expect("valid");
    assert_eq!(t4.token, Token::Pipe);

    let t5 = tokens[5].as_ref().expect("valid");
    assert_eq!(t5.token, Token::Yell);
}

#[test]
fn test_unknown_character_error() {
    let src = "grab @";
    let lexer = Lexer::new(src);
    let tokens: Vec<_> = lexer.collect();

    assert_eq!(tokens.len(), 2);
    assert!(tokens[0].is_ok());

    match &tokens[1] {
        Ok(_) => panic!("Expected error"),
        Err(e) => match e {
            LexError::InvalidToken(span) => assert_eq!(*span, Span::new(5, 6)),
            _ => panic!("Wrong error"),
        }
    }
}

#[test]
fn test_skips_whitespace() {
    let src = "  grab   \n  x";
    let lexer = Lexer::new(src);
    let tokens: Vec<_> = lexer.collect();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].as_ref().unwrap().token, Token::Grab);
    assert_eq!(tokens[1].as_ref().unwrap().token, Token::Ident);
}