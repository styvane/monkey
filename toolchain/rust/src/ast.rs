//! # AST

mod vardecl;
pub(crate) use self::vardecl::LocalVarDecl;

#[derive(Debug)]
pub struct Program {
    /// A program is a sequence of statements.
    pub statements: Vec<Statement>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Statement {
    Var(LocalVarDecl),
}
