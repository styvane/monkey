//! Token type.
//!
//! This module defines the token types.

use std::fmt;
use std::str::FromStr;

/// Token type.
#[derive(Clone, Debug)]
pub struct Token {
    /// The kind of token.
    pub kind: TokenKind,

    /// The literal value of token.
    pub value: Value,

    /// The current span
    pub span: Span,
}

/// Token literal value.
#[derive(Clone, Debug)]
pub struct Value(String);

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
/// The kind of token.
pub enum TokenKind {
    /// This is the end of file.
    EOF,
    /// This is an identifier; for example add, foobar, x, y, ...
    IDENT,
    /// This is an integer.
    INT,
    /// This is a assignment operator.
    ASSIGN,
    /// This a plus (+) operator.
    PLUS,
    /// This is minus (-) operator
    MINUS,
    /// This is a not (!) operator.
    NOT,
    /// This is divide (/) operator.
    DIVIDE,
    /// This is a multiply operator.
    MULTIPLY,
    /// This is an equal operator.
    EQ,
    /// This is a not equal operator.
    NOT_EQ,
    /// This is a greater than operator.
    GT,
    /// This is a lower than operator.
    LT,
    /// This is a comma (,) delimiter.
    COMMA,
    /// This is a semicolon (;) delimiter.
    SEMICOLON,
    /// This is a left parenthesis
    LPARENT,
    /// This is a right parenthesis
    RPARENT,
    /// This is a left brace
    LBRACE,
    /// This is a right brace
    RBRACE,
    /// This is a function keyword
    FUNCTION,
    /// This is a let keyword.
    LET,
}

impl FromStr for TokenKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tkind = match s {
            "EOF" => Self::EOF,
            "IDENT" => Self::IDENT,
            "INT" => Self::INT,
            "=" => Self::ASSIGN,
            "+" => Self::PLUS,
            "-" => Self::MINUS,
            "!" => Self::NOT,
            "*" => Self::MULTIPLY,
            "/" => Self::DIVIDE,
            "," => Self::COMMA,
            ";" => Self::SEMICOLON,
            "(" => Self::LPARENT,
            ")" => Self::RPARENT,
            "{" => Self::LBRACE,
            "}" => Self::RBRACE,
            "FUNCTION" => Self::FUNCTION,
            "LET" => Self::LET,
            _ => return Err("unrecognized token kind"),
        };

        Ok(tkind)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = match self {
            Self::EOF => "EOF",
            Self::IDENT => "IDENT",
            Self::INT => "INT",
            Self::PLUS => "+",
            Self::MINUS => "-",
            Self::MULTIPLY => "*",
            Self::DIVIDE => "/",
            Self::NOT => "!",
            Self::COMMA => ",",
            Self::SEMICOLON => ";",
            Self::LPARENT => "(",
            Self::RPARENT => ")",
            Self::LBRACE => "{",
            Self::RBRACE => "}",
            Self::FUNCTION => "FUNCTION",
            Self::LET => "LET",
            Self::ASSIGN => "=",
            Self::EQ => "==",
            Self::NOT_EQ => "!=",
            Self::LT => "<",
            Self::GT => ">",
        };
        write!(f, "{}", v)
    }
}
impl Token {
    /// Creates new token.
    pub const fn new(kind: TokenKind, value: Value, span: Span) -> Self {
        Self { kind, value, span }
    }
}

#[derive(Debug, Clone)]
/// The Span data represents a region of code associated with an input token.
pub struct Span {
    /// The line number for this token.
    pub lineno: u64,
    /// The column number where this token was found.
    pub column_pos: u64,
}

impl Span {
    /// Creates new span.
    #[inline]
    pub const fn new(lineno: u64, column_pos: u64) -> Self {
        Self { lineno, column_pos }
    }
}
