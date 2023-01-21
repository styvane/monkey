//! Span type.

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
