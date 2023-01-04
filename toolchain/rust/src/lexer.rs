//! Lexer type.
//!
//! This module implement the lexer.

use std::fmt;

/// Lexer type.
pub struct Lexer {
    /// The current input string.
    pub input: Box<dyn Iterator<Item = char>>,
    /// The current line number in the input.
    pub lineno: u64,
    /// The current column position.
    pub column_pos: u64,
}

impl fmt::Debug for Lexer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lexer")
            .field("lineno", &self.lineno)
            .field("column_pos", &self.column_pos)
            .finish()
    }
}
impl Lexer {
    /// Creates new lexer.
    pub const fn new(input: Box<dyn Iterator<Item = char>>) -> Self {
        Self {
            input,
            lineno: 1,
            column_pos: 1,
        }
    }

    /// Eats the whitespace from input.
    pub fn eat_whitespace(mut self) -> Self {
        let iter = self.input.skip_while(|&x| x.is_whitespace());
        self.input = Box::new(iter);
        self
    }
}
