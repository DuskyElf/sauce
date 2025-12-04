use crate::util::span::Span;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum LexError {
    #[error("invalid token at {0:?}")]
    InvalidToken(Span),

    #[error("unterminated string starting at {0:?}")]
    UnterminatedString(Span),
}
