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

#[derive(Debug, Clone)]
/// Punctuation symbol tokens.
pub enum Punctuation {
    /// Assignment symbol.
    Eq,
    /// This a plus (+) operator.
    Plus,
    /// This is minus (-) operator
    Minus,
    /// This is a not (!) operator.
    Not,
    /// This is divide (/) operator.
    Slash,
    /// This is a multiply operator.
    Star,
    /// This is an equal operator.
    EqEq,
    /// This is a not equal operator.
    Ne,
    /// This is a greater than operator.
    Gt,
    /// This is a lower than operator.
    Lt,
    /// This is a comma (,) delimiter.
    Comma,
    /// This is a semicolon (;) delimiter.
    Semi,
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
    Eq,
    /// This a plus (+) operator.
    Plus,
    /// This is minus (-) operator
    Minus,
    /// This is a not (!) operator.
    Not,
    /// This is divide (/) operator.
    Slash,
    /// This is a multiply operator.
    Star,
    /// This is an equal operator.
    EqEq,
    /// This is a not equal operator.
    Ne,
    /// This is a greater than operator.
    Gt,
    /// This is a lower than operator.
    Lt,
    /// This is a comma (,) delimiter.
    Comma,
    /// This is a semicolon (;) delimiter.
    Semi,
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
            "=" => Self::Eq,
            "+" => Self::Plus,
            "-" => Self::Minus,
            "!" => Self::Not,
            "*" => Self::Star,
            "/" => Self::Slash,
            "," => Self::Comma,
            ";" => Self::Semi,
            "(" => Self::Lparen,
            ")" => Self::Rparen,
            "{" => Self::Lbrace,
            "}" => Self::Rbrace,
            "<" => Self::Lt,
            ">" => Self::Gt,
            "==" => Self::EqEq,
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
            Self::Star => "*",
            Self::Slash => "/",
            Self::Not => "!",
            Self::Comma => ",",
            Self::Semi => ";",
            Self::Lparen => "(",
            Self::Rparen => ")",
            Self::Lbrace => "{",
            Self::Rbrace => "}",
            Self::Function => "fn",
            Self::Let => "let",
            Self::Eq => "=",
            Self::EqEq => "==",
            Self::Ne => "!=",
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
#[macro_export]
macro_rules! lookup_keyword {
    ($token_str:expr) => {
        match $token_str {
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "fn" => TokenKind::Function,
            _ => TokenKind::Ident,
        }
    };
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
