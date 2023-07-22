// Package Parser implement the language Parser.
package parser

import (
	"github/com/styvane/monkey/ast"
	"github/com/styvane/monkey/lexer"
	"github/com/styvane/monkey/token"
)

// The Parser parses the input token into an AST.
type Parser struct {
	lexer          *lexer.Lexer
	currentToken   token.Token
	lookaheadToken token.Token
	errors         []ParseError
}

// New instantiate a new parser.
func New(l *lexer.Lexer) *Parser {
	p := &Parser{lexer: l}

	p.nextToken()
	p.nextToken()

	return p
}

// nextToken returns the next token to parse.
func (p *Parser) nextToken() {
	p.currentToken = p.lookaheadToken
	p.lookaheadToken = p.lexer.NextToken()
}

// ParseProgram parses a program into an AST.
func (p *Parser) ParseProgram() *ast.Program {
	program := &ast.Program{}

	for p.currentToken.Kind != token.EOF {
		if stmt := p.parseStatement(); stmt != nil {
			program.Statements = append(program.Statements, stmt)

		}
		p.nextToken()
	}

	return program

}

// ParseStatement parses the next statement in the program.
func (p *Parser) parseStatement() ast.Statement {
	switch p.currentToken.Kind {
	case token.LET:
		return p.parseVariableDecl()
	case token.RETURN:
		return p.ParseReturnStatement()
	default:
		return nil
	}
}

// parseVariableDecl parses a variable declaration statement.
func (p *Parser) parseVariableDecl() ast.Statement {
	stmt := &ast.LocalVariableDecl{Token: p.currentToken}

	if !p.expectedLookaheadToken(token.IDENT) {
		return nil
	}

	stmt.Name = &ast.Identifier{Token: p.currentToken, Value: p.currentToken.Literal}

	if !p.expectedLookaheadToken(token.EQ) {
		return nil
	}

	// TODO: We're skipping the expressions until we encounter a semicolon.
	for !p.currentTokenIs(token.SEMI) {
		p.nextToken()
	}
	return stmt
}

func (p *Parser) currentTokenIs(k token.Kind) bool {
	return p.currentToken.Kind == k
}

func (p *Parser) lookaheadTokenIs(k token.Kind) bool {
	return p.lookaheadToken.Kind == k
}

func (p *Parser) expectedLookaheadToken(k token.Kind) bool {
	if p.lookaheadTokenIs(k) {
		p.nextToken()
		return true

	} else {
		p.errors = append(p.errors, ParseError{k, p.lookaheadToken.Kind})
		return false
	}

}

// Errors returns the slice of parsing errors.
func (p *Parser) Errors() []ParseError {
	return p.errors
}

func (p *Parser) ParseReturnStatement() ast.Statement {
	stmt := &ast.ReturnStatement{Token: p.currentToken}

	p.nextToken()

	// TODO: We're skipping the expression until we encounter a semicolon.

	for !p.currentTokenIs(token.SEMI) {
		p.nextToken()
	}

	return stmt
}
