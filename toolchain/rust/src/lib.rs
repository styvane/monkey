//! Rust toolchain implementation.

#![forbid(clippy::unwrap_used)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    clippy::missing_const_for_fn
)]

pub mod lexer;
pub mod token;
