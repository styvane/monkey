//! This module defines the data structures for an expressions.

/// `ExprData` represents an expression data.
#[derive(Debug, PartialEq, Eq)]
pub enum ExprData {
    VariableDecl(String),
    Return(String),
    ExprStatement(String),
}
