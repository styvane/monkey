//! This module defines the data structure representing a variable declaration
//! statement.

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
