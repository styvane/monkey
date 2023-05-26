//! The error module defines the parsing errors.

use std::error::Error as StdError;
use std::fmt;

use crate::token::TokenKind;

/// Error type.
#[derive(Debug)]
pub enum Error {
    /// The error type when an unexpected token is encountered.
    SyntaxError {
        expected: TokenKind,
        found: TokenKind,
    },
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::SyntaxError { expected, found } => {
                format!("unexpected : '{expected}\nfound: '{found}'")
            }
        };
        write!(f, "{}", value)
    }
}
