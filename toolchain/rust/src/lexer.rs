//! Lexer type.
//!
//! This module implement the lexer.

use std::fmt::{self, Write};
use std::iter::Peekable;
use std::str::CharIndices;

use crate::token::{lookup_keyword, Span, Token, TokenKind, Value};

/// Lexer type.
pub struct Lexer<I>
where
    I: Iterator<Item = (usize, char)>,
{
    /// The current input string.
    pub chars: Peekable<I>,
    /// The current line number in the input.
    pub lineno: usize,
}

impl<I> fmt::Debug for Lexer<I>
where
    I: Iterator<Item = (usize, char)>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lexer")
            .field("lineno", &self.lineno)
            .finish()
    }
}

impl<'a> Lexer<CharIndices<'a>> {
    /// Creates new lexer with given string input.
    pub fn from_text(input: &'a str) -> Lexer<CharIndices> {
        let chars = input.char_indices().peekable();
        Self { chars, lineno: 1 }
    }
}

impl<I> Lexer<I>
where
    I: Iterator<Item = (usize, char)>,
{
    /// Eats the whitespace from input.
    fn eat_whitespace(&mut self) {
        while self.peek_char(|&x| x.is_whitespace()).is_some() {}
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespace();
        let value: Value;
        let kind: TokenKind;
        let (position, literal) = match self.chars.next() {
            Some((pos, ch)) => (pos, ch),
            _ => {
                value = Value::new("".into());
                let span = Span::new(self.lineno, 0);
                return Some(Token::new(TokenKind::Eof, value, span));
            }
        };

        if is_identifier(&literal) {
            let mut ident = String::from(literal);
            if let Some(value) = self.lex_identifier() {
                if write!(&mut ident, "{}", value).is_err() {
                    return None;
                }
            }
            kind = lookup_keyword(&ident);
            value = Value::new(ident)
        } else if literal == '=' {
            kind = match self.peek_char(|&x| x == '=') {
                Some((_, ch)) => {
                    value = Value::new(format!("{literal}{ch}"));
                    TokenKind::Eq
                }
                _ => {
                    value = Value::new(literal.into());
                    TokenKind::Assign
                }
            };
        } else if literal == '!' {
            kind = match self.peek_char(|&x| x == '=') {
                Some((_, ch)) => {
                    value = Value::new(format!("{literal}{ch}"));
                    TokenKind::NotEq
                }
                _ => {
                    value = Value::new(literal.into());
                    TokenKind::Not
                }
            };
        } else if literal.is_ascii_digit() {
            let mut digits = String::from(literal);
            if let Some(extra_digits) = self.lex_int() {
                if write!(&mut digits, "{}", extra_digits).is_err() {
                    return None;
                }
            }
            value = Value::new(digits);
            kind = TokenKind::Int;
        } else {
            let literal = literal.to_string();
            kind = literal.parse().ok()?;
            value = Value::new(literal);
        }
        let span = Span::new(self.lineno, position);
        Some(Token::new(kind, value, span))
    }

    /// Returns the identitifer.
    fn lex_identifier(&mut self) -> Option<String> {
        let mut ident = String::new();
        while let Some((_, ch)) = self.peek_char(is_identifier) {
            if write!(&mut ident, "{}", ch).is_err() {
                return None;
            }
        }

        Some(ident)
    }

    /// Inspect next element.
    fn peek_char(&mut self, func: impl FnOnce(&char) -> bool) -> Option<(usize, char)> {
        self.chars.next_if(|(_, c)| func(c))
    }

    /// Return a digit.
    fn lex_int(&mut self) -> Option<String> {
        let mut digits = String::new();

        while let Some((_, ch)) = self.peek_char(|&x| x.is_ascii_digit()) {
            if write!(&mut digits, "{}", ch).is_err() {
                return None;
            }
        }
        Some(digits)
    }
}

/// Returns true if the character is a letter or underscore.
fn is_identifier(c: &char) -> bool {
    c.is_alphabetic() || *c == '_'
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::TokenKind;

    #[test]
    fn create_lexemes_successfully() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 4;
if (5 < 10) {
return true;
} else {
return false;
}
10 == 10;
10 != 9;
let delta = 9;
let snow = 9;"#;

        #[derive(Debug)]
        struct Case<'a> {
            /// Expected token kind
            expected_kind: TokenKind,
            /// token literal
            value: &'a str,
        }

        impl<'a> Case<'a> {
            /// Create new test case
            const fn new(expected_kind: TokenKind, value: &'a str) -> Self {
                Self {
                    expected_kind,
                    value,
                }
            }
        }

        use crate::token::TokenKind::*;

        let tests = [
            (Let, "let"),
            (Ident, "five"),
            (Assign, "="),
            (Int, "5"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "ten"),
            (Assign, "="),
            (Int, "10"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "add"),
            (Assign, "="),
            (Function, "fn"),
            (Lparen, "("),
            (Ident, "x"),
            (Comma, ","),
            (Ident, "y"),
            (Rparen, ")"),
            (Lbrace, "{"),
            (Ident, "x"),
            (Plus, "+"),
            (Ident, "y"),
            (Semicolon, ";"),
            (Rbrace, "}"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "result"),
            (Assign, "="),
            (Ident, "add"),
            (Lparen, "("),
            (Ident, "five"),
            (Comma, ","),
            (Ident, "ten"),
            (Rparen, ")"),
            (Semicolon, ";"),
            (Not, "!"),
            (Minus, "-"),
            (Divide, "/"),
            (Multiply, "*"),
            (Int, "5"),
            (Semicolon, ";"),
            (Int, "5"),
            (Lt, "<"),
            (Int, "10"),
            (Gt, ">"),
            (Int, "4"),
            (Semicolon, ";"),
            (If, "if"),
            (Lparen, "("),
            (Int, "5"),
            (Lt, "<"),
            (Int, "10"),
            (Rparen, ")"),
            (Lbrace, "{"),
            (Return, "return"),
            (True, "true"),
            (Semicolon, ";"),
            (Rbrace, "}"),
            (Else, "else"),
            (Lbrace, "{"),
            (Return, "return"),
            (False, "false"),
            (Semicolon, ";"),
            (Rbrace, "}"),
            (Int, "10"),
            (Eq, "=="),
            (Int, "10"),
            (Semicolon, ";"),
            (Int, "10"),
            (NotEq, "!="),
            (Int, "9"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "delta"),
            (Assign, "="),
            (Int, "9"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "snow"),
            (Assign, "="),
            (Int, "9"),
            (Semicolon, ";"),
            (Eof, ""),
        ]
        .into_iter()
        .map(|(kind, val)| Case::new(kind, val));

        let mut lexer = Lexer::from_text(input);
        for (index, case) in tests.enumerate() {
            let token = lexer.next_token().expect("failed to create lexeme");
            assert_eq!(
                case.expected_kind, token.kind,
                "{index}: {case:?} {token:?}"
            );
            assert_eq!(
                case.value,
                token.value.literal(),
                "{index}: {case:?} {token:?}"
            );
        }
    }
}
