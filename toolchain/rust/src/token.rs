//! Token type.
//!
//! This module defines the token types.

use super::span::Span;
use super::token_kind::TokenKind;

#[derive(Clone, Debug)]
/// Token type.
pub struct Token {
    /// Token value.
    pub value: TokenValue,

    /// The kind of token.
    pub kind: TokenKind,

    /// Span associated witht this token.
    pub span: Span,
}

/// Token literal value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenValue {
    /// An unknown token.
    Unknown(char),

    /// An identifier or keyword.
    Word(String),

    /// A delimiter token: `{`, `}`, `[`, `]`, `(`, or `)`.
    Delimiter(char),

    /// A Comma token: `,`.
    Comma,

    /// A semicolon token: `;`
    Semi,

    /// A number. Only integers are currently supported.
    Number(String),

    /// An operator: `_`, `*`, ...
    Operator(String),

    /// End of file,
    Eof,
}

impl Token {
    /// Creates new token.
    pub const fn new(value: TokenValue, kind: TokenKind, span: Span) -> Self {
        Self { value, span, kind }
    }
}
