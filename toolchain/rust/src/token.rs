//! Token type.
//!
//! This module defines the token types.

mod kind;

use std::borrow::Cow;

pub(super) use self::kind::TokenKind;
use super::span::Span;

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

/// Token value.
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

    /// Returns a string value of the token.
    pub fn as_str(&self) -> Cow<'_, str> {
        match &self.value {
            TokenValue::Unknown(c) | TokenValue::Delimiter(c) => Cow::from(c.to_string()),
            TokenValue::Word(s) | TokenValue::Operator(s) | TokenValue::Number(s) => Cow::from(s),
            TokenValue::Comma => Cow::from(","),
            TokenValue::Semi => Cow::from(";"),
            TokenValue::Eof => Cow::from(""),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.kind == other.kind
    }
}

impl Eq for Token {}
