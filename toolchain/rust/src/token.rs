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

impl Value {
    /// Creates new token value.
    pub const fn new(value: String) -> Self {
        Value(value)
    }

    #[inline]
    /// Returns a reference to the inner literal value.
    pub fn literal(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The kind of token.
pub enum TokenKind {
    /// This is the end of file.
    Eof,
    /// This is an identifier; for example add, foobar, x, y, ...
    Ident,
    /// This is an integer.
    Int,
    /// This is a assignment operator.
    Assign,
    /// This a plus (+) operator.
    Plus,
    /// This is minus (-) operator
    Minus,
    /// This is a not (!) operator.
    Not,
    /// This is divide (/) operator.
    Divide,
    /// This is a multiply operator.
    Multiply,
    /// This is an equal operator.
    Eq,
    /// This is a not equal operator.
    NotEq,
    /// This is a greater than operator.
    Gt,
    /// This is a lower than operator.
    Lt,
    /// This is a comma (,) delimiter.
    Comma,
    /// This is a semicolon (;) delimiter.
    Semicolon,
    /// This is a left parenthesis
    Lparen,
    /// This is a right parenthesis
    Rparen,
    /// This is a left brace
    Lbrace,
    /// This is a right brace
    Rbrace,
    /// This is a function keyword
    Function,
    /// This is a let keyword.
    Let,
    /// This is the "if" keyword.
    If,
    /// This is the "else" keyword.
    Else,
    /// This is the "return" keyword.
    Return,
    /// This is the "true" keyword.
    True,
    /// This is the "false" keyword.
    False,
}

impl FromStr for TokenKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tkind = match s {
            "" => Self::Eof,
            "Ident" => Self::Ident,
            "Int" => Self::Int,
            "=" => Self::Assign,
            "+" => Self::Plus,
            "-" => Self::Minus,
            "!" => Self::Not,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            "," => Self::Comma,
            ";" => Self::Semicolon,
            "(" => Self::Lparen,
            ")" => Self::Rparen,
            "{" => Self::Lbrace,
            "}" => Self::Rbrace,
            "<" => Self::Lt,
            ">" => Self::Gt,
            "Function" => Self::Function,
            "Let" => Self::Let,
            "If" => Self::If,
            "Else" => Self::Else,
            "True" => Self::True,
            "False" => Self::False,
            "Return" => Self::Return,
            _ => {
                return Err("unrecognized token kind");
            }
        };

        Ok(tkind)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = match self {
            Self::Eof => "Eof",
            Self::Ident => "Ident",
            Self::Int => "Int",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Not => "!",
            Self::Comma => ",",
            Self::Semicolon => ";",
            Self::Lparen => "(",
            Self::Rparen => ")",
            Self::Lbrace => "{",
            Self::Rbrace => "}",
            Self::Function => "fn",
            Self::Let => "let",
            Self::Assign => "=",
            Self::Eq => "==",
            Self::NotEq => "!=",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::If => "if",
            Self::Else => "else",
            Self::True => "true",
            Self::False => "false",
            Self::Return => "return",
        };
        write!(f, "{}", v)
    }
}

/// Keyword table.
pub static KEYWORDS: phf::Map<&'static str, TokenKind> = phf::phf_map! {
    "let" => TokenKind::Let,
    "if"  => TokenKind::If,
    "true" => TokenKind::True,
    "false" => TokenKind::False,
    "else" =>  TokenKind::Else,
    "return" =>  TokenKind::Return,
    "fn" => TokenKind::Function,
};

/// Lookups keyword.
pub fn lookup_keyword(ident: &str) -> TokenKind {
    KEYWORDS.get(ident).cloned().unwrap_or(TokenKind::Ident)
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
    pub lineno: usize,
    /// The column number where this token was found.
    pub column_pos: usize,
}

impl Span {
    /// Creates new span.
    #[inline]
    pub const fn new(lineno: usize, column_pos: usize) -> Self {
        Self { lineno, column_pos }
    }
}
