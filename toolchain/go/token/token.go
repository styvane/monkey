// Package implements the token data structure and operations.
package token

const (
	ILLEGAL = "ILLEGAL"
	EOF     = "EOF"

	// Identifiers and literals
	IDENT = "IDENT" // add, foobar, x, y ...
	INT   = "INT"   // 123456

	// Operators
	ASSIGN   = "="
	PLUS     = "+"
	MINUS    = "-"
	BANG     = "!"
	ASTERISK = "*"
	SLASH    = "/"

	LT     = "<"
	GT     = ">"
	EQ     = "=="
	NOT_EQ = "!="

	// Delimiters
	COMMA     = ","
	SEMICOLON = ";"
	LPAREN    = "("
	RPAREN    = ")"
	LBRACE    = "{"
	RBRACE    = "{"

	// Keywords
	FUNCTION = "FUNCTION"
	LET      = "LET"
	IF       = "IF"
	ELSE     = "ELSE"
	TRUE     = "TRUE"
	FALSE    = "FALSE"
	RETURN   = "RETURN"
)

// Keywords table.
var keywords = map[string]TokenType{
	"fn":     FUNCTION,
	"let":    LET,
	"if":     IF,
	"else":   ELSE,
	"return": RETURN,
	"true":   TRUE,
	"false":  FALSE,
}

// The Token type represents a lexical token.
type Token struct {
	Type    TokenType
	Literal string
	Span
}

// Span represents a region of code.
type Span struct {
	// Lineno is the line number in the input.
	Lineno int
	// Position is the postion in the input.
	Position int
}

// NewSpan creates a new span with the line number and position.
func NewSpan(lineno, position int) Span {
	return Span{Lineno: lineno, Position: position}
}

// TokenType represents the type of a token.
type TokenType string

// NewToken create token.
func NewToken(tokenType TokenType, ch rune, span Span) Token {
	return Token{Type: tokenType, Literal: string(ch), Span: span}
}

// LookupIdent lookup and identifier in input
func LookupIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}
