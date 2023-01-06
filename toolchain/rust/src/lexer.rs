//! Lexer type.
//!
//! This module implement the lexer.

use std::fmt::{self, Write};
use std::iter::Peekable;
use std::str::Chars;

use crate::token::{lookup_keyword, Span, Token, TokenKind, Value};

/// Lexer type.
pub struct Lexer<'a> {
    /// The current input string.
    pub input: Peekable<Chars<'a>>,
    /// The current line number in the input.
    pub lineno: u64,
    /// The current column position.
    pub column_pos: u64,
}

impl<'a> fmt::Debug for Lexer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lexer")
            .field("lineno", &self.lineno)
            .field("column_pos", &self.column_pos)
            .finish()
    }
}

impl<'a> Lexer<'a> {
    /// Creates new lexer.
    pub const fn new(input: Peekable<Chars<'a>>) -> Self {
        Self {
            input,
            lineno: 1,
            column_pos: 1,
        }
    }

    /// Eats the whitespace from input.
    pub fn eat_whitespace(&mut self) {
        while self.lookahead(|x| x.is_whitespace()).is_some() {}
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespace();
        let value: Value;
        let kind: TokenKind;
        let literal = match self.input.next() {
            Some(ch) => ch,
            _ => {
                value = Value::new("".into());
                let span = Span::new(self.lineno, self.column_pos);
                return Some(Token::new(TokenKind::EOF, value, span));
            }
        };

        if Self::is_identifier(&literal) {
            let mut ident = String::from(literal);
            if let Some(value) = self.parse_identifier() {
                if write!(&mut ident, "{}", value).is_err() {
                    return None;
                }
            }
            kind = lookup_keyword(&ident);
            value = Value::new(ident)
        } else if literal == '=' {
            kind = match self.lookahead(|&x| x == '=') {
                Some(ch) => {
                    value = Value::new(format!("{literal}{ch}"));
                    TokenKind::EQ
                }
                _ => {
                    value = Value::new(literal.into());
                    TokenKind::ASSIGN
                }
            };
        } else if literal == '!' {
            kind = match self.lookahead(|&x| x == '=') {
                Some(ch) => {
                    value = Value::new(format!("{literal}{ch}"));
                    TokenKind::NOT_EQ
                }
                _ => {
                    value = Value::new(literal.into());
                    TokenKind::NOT
                }
            };
        } else if literal.is_ascii_digit() {
            let mut digits = String::from(literal);
            if let Some(extra_digits) = self.parse_int() {
                if write!(&mut digits, "{}", extra_digits).is_err() {
                    return None;
                }
            }
            value = Value::new(digits);
            kind = TokenKind::INT;
        } else {
            let literal = literal.to_string();
            kind = literal.parse().ok()?;
            value = Value::new(literal);
        }
        self.column_pos += 1;
        let span = Span::new(self.lineno, self.column_pos);
        Some(Token::new(kind, value, span))
    }

    /// Returns the identitifer.
    fn parse_identifier(&mut self) -> Option<String> {
        let mut ident = String::new();
        while let Some(ch) = self.lookahead(Self::is_identifier) {
            if write!(&mut ident, "{}", ch).is_err() {
                return None;
            }
        }

        Some(ident)
    }

    /// Returns true if the character is a letter or underscore.
    fn is_identifier(c: &char) -> bool {
        c.is_alphabetic() || *c == '_'
    }

    /// Inspect next element.
    fn lookahead(&mut self, func: impl FnOnce(&char) -> bool) -> Option<char> {
        self.input.next_if(func)
    }

    /// Return a digit.
    fn parse_int(&mut self) -> Option<String> {
        let mut digits = String::new();

        while let Some(ch) = self.lookahead(|x| x.is_ascii_digit()) {
            if write!(&mut digits, "{}", ch).is_err() {
                return None;
            }
        }
        Some(digits)
    }
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
            (LET, "let"),
            (IDENT, "five"),
            (ASSIGN, "="),
            (INT, "5"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "ten"),
            (ASSIGN, "="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "add"),
            (ASSIGN, "="),
            (FUNCTION, "fn"),
            (LPAREN, "("),
            (IDENT, "x"),
            (COMMA, ","),
            (IDENT, "y"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (IDENT, "x"),
            (PLUS, "+"),
            (IDENT, "y"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "result"),
            (ASSIGN, "="),
            (IDENT, "add"),
            (LPAREN, "("),
            (IDENT, "five"),
            (COMMA, ","),
            (IDENT, "ten"),
            (RPAREN, ")"),
            (SEMICOLON, ";"),
            (NOT, "!"),
            (MINUS, "-"),
            (DIVIDE, "/"),
            (MULTIPLY, "*"),
            (INT, "5"),
            (SEMICOLON, ";"),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (GT, ">"),
            (INT, "4"),
            (SEMICOLON, ";"),
            (IF, "if"),
            (LPAREN, "("),
            (INT, "5"),
            (LT, "<"),
            (INT, "10"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (TRUE, "true"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (ELSE, "else"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (FALSE, "false"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (INT, "10"),
            (EQ, "=="),
            (INT, "10"),
            (SEMICOLON, ";"),
            (INT, "10"),
            (NOT_EQ, "!="),
            (INT, "9"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "delta"),
            (ASSIGN, "="),
            (INT, "9"),
            (SEMICOLON, ";"),
            (LET, "let"),
            (IDENT, "snow"),
            (ASSIGN, "="),
            (INT, "9"),
            (SEMICOLON, ";"),
            (EOF, ""),
        ]
        .into_iter()
        .map(|(kind, val)| Case::new(kind, val));

        let mut lexer = Lexer::new(input.chars().peekable());
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
