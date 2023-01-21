//! Monkey interpreter.

use std::io;

use monkey::repl;

fn main() {
    println!("Welcome to the Monkey programming language!");
    repl::start(io::stdout()).expect("failed to readline");
}
