//! This module defines the data structure representing code syntax.

use crate::expr::ExprData;
use crate::token::Token;

/// `LocalVardecl` represents a variable declaration.
#[derive(Debug, PartialEq, Eq)]
pub struct LocalVarDecl {
    /// This field is the token introducing the variable declaration.
    pub token: Token,
    /// `Name` is the name of the identifier.
    pub name: Token,
    /// This is the expression value.
    pub expr: ExprData,
}

/// `Return` represents a return statement.
#[derive(Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    /// `return` token.
    pub token: Token,
    /// returned expresssion
    pub expr: ExprData,
}
