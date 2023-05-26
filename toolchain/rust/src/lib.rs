//! Rust toolchain implementation.

#![forbid(clippy::unwrap_used)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    clippy::missing_const_for_fn
)]

mod ast;
mod error;
mod expr;
pub mod lexer;
pub mod parser;
pub mod repl;
mod span;
pub mod token;
