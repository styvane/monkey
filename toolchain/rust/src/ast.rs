//! # AST

pub(super) mod syntax;

use self::syntax::*;

#[derive(Debug)]
pub struct Program {
    /// A program is a sequence of statements.
    pub statements: Vec<Statement>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
    Var(LocalVarDecl),
    Return(ReturnStatement),
    Expr(ExprStatement),
}
