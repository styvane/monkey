//! Read Eval Print Loop.

use std::io;

use rustyline::{Editor, Result};

use crate::lexer::Lexer;
use crate::token::TokenKind;

const PROMPT: &str = ">> ";

/// Starts the REPL.
pub fn start(mut writer: impl io::Write) -> Result<()> {
    let mut line_reader = Editor::<()>::new()?;
    while let Ok(line) = line_reader.readline(PROMPT) {
        let mut lexer = Lexer::from_text(&line);
        while let Some(tok) = lexer.next_token() {
            if tok.kind == TokenKind::Eof {
                break;
            }
            write!(&mut writer, "{:?}", tok).expect("failed to write");
            writer.flush()?;
        }
    }
    Ok(())
}
