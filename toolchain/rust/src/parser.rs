//! Parser
//!
//! This module implement the parser for the language.

use crate::ast::syntax::*;
use crate::ast::{Program, Statement};
use crate::error::Error;
use crate::expr::{Expr, ExprData};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

/// Parser type.
#[derive(Debug)]
pub struct Parser<I: Iterator<Item = (usize, char)>> {
    lexer: Lexer<I>,
    current_token: Option<Token>,
    lookahead_token: Option<Token>,
    errors: Vec<Error>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = (usize, char)>,
{
    /// Instantiates new parser.
    pub fn new(mut lexer: Lexer<I>) -> Self {
        let current_token = lexer.next_token();
        let lookahead_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            lookahead_token,
            errors: Vec::new(),
        }
    }

    /// Advances the parser to next tokens.
    fn advance(&mut self) {
        self.current_token = self.lookahead_token.take();
        self.lookahead_token = self.lexer.next_token();
    }

    /// Parse the program.
    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::with_capacity(500);

        while let Some(tok) = &self.current_token {
            if tok.kind.as_str().is_empty() {
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.advance();
        }

        Program { statements }
    }

    /// Returns true if the lookahead token as the expected type.
    fn is_valid_lookahead_token(&mut self, expected: TokenKind) -> bool {
        match &self.lookahead_token {
            Some(tok) => {
                if tok.kind != expected {
                    self.errors.push(Error::SyntaxError {
                        expected,
                        found: tok.kind,
                    });
                }
                tok.kind == expected
            }
            _ => true,
        }
    }

    /// Returns true if the current token as the expected type.
    fn is_valid_current_token(&mut self, expected: TokenKind) -> bool {
        match &self.current_token {
            Some(tok) => {
                if tok.kind != expected {
                    self.errors.push(Error::SyntaxError {
                        expected,
                        found: tok.kind,
                    });
                }
                tok.kind == expected
            }
            _ => true,
        }
    }

    /// Advances the parser if the next token is encountered.
    fn advance_next_if(&mut self, next: TokenKind) -> Option<()> {
        self.is_valid_lookahead_token(next).then(|| self.advance())
    }

    /// Parses a statement.
    fn parse_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.as_ref()?;
        match token.kind {
            TokenKind::Let => self.parse_var_decl(),
            TokenKind::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_var_decl(&mut self) -> Option<Statement> {
        let token = self.current_token.take()?;
        self.advance_next_if(TokenKind::Ident)?;
        let name = self.current_token.take()?;
        // TODO: skip expression parsing.
        while !self.is_valid_current_token(TokenKind::Semi) {
            self.advance();
        }
        let stmt = Statement::Var(LocalVarDecl {
            token,
            name,
            expr: ExprData::VariableDecl(Expr, "".into()),
        });

        Some(stmt)
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.take()?;
        if !self.is_valid_current_token(TokenKind::Semi) {
            self.advance();
        }
        let stmt = Statement::Return(ReturnStatement {
            token,
            expr: ExprData::Return(Expr, "".into()),
        });
        Some(stmt)
    }
}

/// `PrattParser` specifies the mechanism for parsing a token type
/// using PRATTER PARSER.
pub trait PrattParser {
    /// Parse token if its found in the prefix position.
    fn prefix_parse() -> ExprData;
    /// Parse a token if its found in an infix position.
    fn infix_parse(ast: ExprData) -> ExprData;
}

#[cfg(test)]
mod tests {
    use crate::ast::Statement;
    use crate::error::Error;
    use crate::lexer::Lexer;

    use super::Parser;

    #[test]
    fn parse_var_decl() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 999999;
"#;

        let lexer = Lexer::from_text(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        assert_eq!(program.statements.len(), 3);
        check_parser_errors(&parser.errors);

        let tests = vec!["x", "y", "foobar"];

        for (index, test) in tests.iter().enumerate() {
            let stmt = &program.statements[index];
            check_vardecl_statement(stmt, test);
        }
    }

    fn check_vardecl_statement(statement: &Statement, name: &str) {
        let Statement::Var(decl) = statement else { panic!("expected variable declaration found: {:?}", statement)};

        assert_eq!(decl.token.kind.as_str(), "let");
        assert_eq!(decl.name.as_str(), name);
    }

    fn check_parser_errors(errors: &[Error]) {
        if errors.is_empty() {
            return;
        }

        for err in errors {
            eprintln!("{err}");
        }
        panic!("error: could not compile due to previous error");
    }

    #[test]
    fn parse_return_stmt() {
        let input = r#"
return 5;
return add(3, 1);
return 999999;
"#;

        let lexer = Lexer::from_text(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse();
        assert_eq!(program.statements.len(), 3);
        check_parser_errors(&parser.errors);

        for stmt in &program.statements {
            assert!(matches!(stmt, Statement::Return(_)))
        }
    }
}
