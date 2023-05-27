//! This module defines the data structures for an expressions.

/// Expr represents an expression.
#[derive(Debug, PartialEq, Eq)]
pub struct Expr;

/// `ExprData` represents an expression data.
#[derive(Debug, PartialEq, Eq)]
pub enum ExprData {
    VariableDecl(Expr, String),
    Return(Expr, String),
}
