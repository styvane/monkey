//! Lexer type.
//!
//! This module implement the lexer.

use std::fmt;

use std::iter::Peekable;
use std::str::CharIndices;

use super::span::Span;
use super::token::{Token, TokenKind, TokenValue};

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

/// Matches an operator.
macro_rules! operator {
    () => {
        '+' | '-' | '*' | '/' | '!' | '=' | '<' | '>'
    };
}

/// Matches a delimiter
macro_rules! delimiter {
    () => {
        '{' | '}' | '(' | ')' | '[' | ']'
    };
}

/// Return the kind of delimiter.
macro_rules! delimiter_kind {
    ($del:expr) => {
        match $del {
            '{' => TokenKind::Lbrace,
            '}' => TokenKind::Rbrace,
            '(' => TokenKind::Lparen,
            ')' => TokenKind::Rparen,
            '[' => TokenKind::Lbracket,
            ']' => TokenKind::Rbracket,
            _ => unreachable!(),
        }
    };
}

/// Lookup keyword.
macro_rules! lookup_keyword {
    ($word:expr) => {
        match $word.as_str() {
            "let" => TokenKind::Let,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "fn" => TokenKind::Function,
            _ => TokenKind::Ident,
        }
    };
}

impl<I> Lexer<I>
where
    I: Iterator<Item = (usize, char)>,
{
    /// Eats the whitespace from input.
    fn eat_whitespace(&mut self) {
        while self.lookahead(|&x| x.is_whitespace()).is_some() {}
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespace();

        let mut token = Token::new(TokenValue::Eof, TokenKind::Eof, Span::new(self.lineno, 0));
        let Some((position, literal)) =  self.chars.next() else { return Some(token) };

        token.span = Span::new(self.lineno, position);

        match literal {
            ',' => {
                token.value = TokenValue::Comma;
                token.kind = TokenKind::Comma;
            }
            ';' => {
                token.value = TokenValue::Semi;
                token.kind = TokenKind::Semi;
            }
            operator!() => {
                if literal == '=' {
                    if let Some((_, ch)) = self.lookahead(|&x| x == '=') {
                        token.value = TokenValue::Operator(format!("{literal}{ch}"));
                        token.kind = TokenKind::EqEq;
                    } else {
                        token.value = TokenValue::Operator(literal.into());
                        token.kind = TokenKind::Eq;
                    }
                } else if literal == '!' {
                    if let Some((_, ch)) = self.lookahead(|&x| x == '=') {
                        token.value = TokenValue::Operator(format!("{literal}{ch}"));
                        token.kind = TokenKind::Ne;
                    } else {
                        token.value = TokenValue::Operator(literal.into());
                        token.kind = TokenKind::Not;
                    }
                } else {
                    let literal: String = literal.into();
                    let kind = TokenKind::from(literal.as_str());
                    token.value = TokenValue::Operator(literal);
                    token.kind = kind;
                }
            }
            delimiter!() => {
                token.value = TokenValue::Delimiter(literal);
                token.kind = delimiter_kind!(literal);
            }
            _ => {
                if is_identifier(&literal) {
                    let mut ident = String::from(literal);
                    if let Some(value) = self.lex_identifier() {
                        ident.push_str(&value);
                    }

                    let kind = lookup_keyword!(ident);
                    token.value = TokenValue::Word(ident);
                    token.kind = kind;
                } else if literal.is_ascii_digit() {
                    let mut digits = String::from(literal);
                    if let Some(extra_digits) = self.lex_int() {
                        digits.push_str(&extra_digits);
                    }
                    token = Token::new(
                        TokenValue::Number(digits),
                        TokenKind::Number,
                        Span::new(self.lineno, position),
                    );
                } else {
                    token.value = TokenValue::Unknown(literal);
                    token.kind = TokenKind::Unknown;
                }
            }
        };
        Some(token)
    }

    /// Returns the identitifer.
    fn lex_identifier(&mut self) -> Option<String> {
        let mut ident = String::new();
        while let Some((_, ch)) = self.lookahead(is_identifier) {
            ident.push(ch);
        }
        if ident.is_empty() {
            return None;
        }
        Some(ident)
    }

    /// Inspect next element.
    fn lookahead(&mut self, func: impl FnOnce(&char) -> bool) -> Option<(usize, char)> {
        self.chars.next_if(|(_, c)| func(c))
    }

    /// Return a digit.
    fn lex_int(&mut self) -> Option<String> {
        let mut digits = String::new();
        while let Some((_, ch)) = self.lookahead(|&x| x.is_ascii_digit()) {
            digits.push(ch);
        }
        if digits.is_empty() {
            return None;
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
    use crate::token::TokenValue;

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
let nine = 9;
let snow = 9;"#;

        #[derive(Debug)]
        struct Case {
            /// Token value
            expected_value: TokenValue,
            /// Expected token kind
            expected_kind: TokenKind,
        }

        impl Case {
            /// Create new test case
            const fn new(expected_value: TokenValue, expected_kind: TokenKind) -> Self {
                Self {
                    expected_value,
                    expected_kind,
                }
            }
        }

        use super::TokenKind::*;
        let tests = [
            (TokenValue::Word("let".into()), Let),
            (TokenValue::Word("five".into()), Ident),
            (TokenValue::Operator("=".into()), Eq),
            (TokenValue::Number("5".into()), Number),
            (TokenValue::Semi, Semi),
            (TokenValue::Word("let".into()), Let),
            (TokenValue::Word("ten".into()), Ident),
            (TokenValue::Operator("=".into()), Eq),
            (TokenValue::Number("10".into()), Number),
            (TokenValue::Semi, Semi),
            (TokenValue::Word("let".into()), Let),
            (TokenValue::Word("add".into()), Ident),
            (TokenValue::Operator("=".into()), Eq),
            (TokenValue::Word("fn".into()), Function),
            (TokenValue::Delimiter('('), Lparen),
            (TokenValue::Word("x".into()), Ident),
            (TokenValue::Comma, Comma),
            (TokenValue::Word("y".into()), Ident),
            (TokenValue::Delimiter(')'), Rparen),
            (TokenValue::Delimiter('{'), Lbrace),
            (TokenValue::Word("x".into()), Ident),
            (TokenValue::Operator("+".into()), Plus),
            (TokenValue::Word("y".into()), Ident),
            (TokenValue::Semi, Semi),
            (TokenValue::Delimiter('}'), Rbrace),
            (TokenValue::Semi, Semi),
            (TokenValue::Word("let".into()), Let),
            (TokenValue::Word("result".into()), Ident),
            (TokenValue::Operator("=".into()), Eq),
            (TokenValue::Word("add".into()), Ident),
            (TokenValue::Delimiter('('), Lparen),
            (TokenValue::Word("five".into()), Ident),
            (TokenValue::Comma, Comma),
            (TokenValue::Word("ten".into()), Ident),
            (TokenValue::Delimiter(')'), Rparen),
            (TokenValue::Semi, Semi),
            (TokenValue::Operator("!".into()), Not),
            (TokenValue::Operator("-".into()), Minus),
            (TokenValue::Operator("/".into()), Slash),
            (TokenValue::Operator("*".into()), Star),
            (TokenValue::Number("5".into()), Number),
            (TokenValue::Semi, Semi),
            (TokenValue::Number("5".into()), Number),
            (TokenValue::Operator("<".into()), Lt),
            (TokenValue::Number("10".into()), Number),
            (TokenValue::Operator(">".into()), Gt),
            (TokenValue::Number("4".into()), Number),
            (TokenValue::Semi, Semi),
            (TokenValue::Word("if".into()), If),
            (TokenValue::Delimiter('('), Lparen),
            (TokenValue::Number("5".into()), Number),
            (TokenValue::Operator("<".into()), Lt),
            (TokenValue::Number("10".into()), Number),
            (TokenValue::Delimiter(')'), Rparen),
            (TokenValue::Delimiter('{'), Lbrace),
            (TokenValue::Word("return".into()), Return),
            (TokenValue::Word("true".into()), True),
            (TokenValue::Semi, Semi),
            (TokenValue::Delimiter('}'), Rbrace),
            (TokenValue::Word("else".into()), Else),
            (TokenValue::Delimiter('{'), Lbrace),
            (TokenValue::Word("return".into()), Return),
            (TokenValue::Word("false".into()), False),
            (TokenValue::Semi, Semi),
            (TokenValue::Delimiter('}'), Rbrace),
            (TokenValue::Number("10".into()), Number),
            (TokenValue::Operator("==".into()), EqEq),
            (TokenValue::Number("10".into()), Number),
            (TokenValue::Semi, Semi),
            (TokenValue::Number("10".into()), Number),
            (TokenValue::Operator("!=".into()), Ne),
            (TokenValue::Number("9".into()), Number),
            (TokenValue::Semi, Semi),
            (TokenValue::Word("let".into()), Let),
            (TokenValue::Word("nine".into()), Ident),
            (TokenValue::Operator("=".into()), Eq),
            (TokenValue::Number("9".into()), Number),
            (TokenValue::Semi, Semi),
            (TokenValue::Word("let".into()), Let),
            (TokenValue::Word("snow".into()), Ident),
            (TokenValue::Operator("=".into()), Eq),
            (TokenValue::Number("9".into()), Number),
            (TokenValue::Semi, Semi),
            (TokenValue::Eof, Eof),
        ]
        .into_iter()
        .map(|(v, k)| Case::new(v, k));

        let mut lexer = Lexer::from_text(input);
        for (index, case) in tests.enumerate() {
            let token = lexer.next_token().expect("failed to create lexeme");
            assert_eq!(
                case.expected_value, token.value,
                "{index}: {case:?} {token:?}"
            );
            assert_eq!(
                case.expected_kind, token.kind,
                "{index}: {case:?} {token:?}"
            );
        }
    }
}
