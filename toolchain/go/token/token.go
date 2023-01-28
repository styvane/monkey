// Package implements the token data structure and operations.
package token

const (
	UNKOWN = "UNKNOWN"
	EOF    = "EOF"

	// Identifiers and literals
	IDENT  = "IDENT"  // add, foobar, x, y ...
	NUMBER = "NUMBER" // 123456

	// Operators
	EQ       = "="
	PLUS     = "+"
	MINUS    = "-"
	NOT      = "!"
	ASTERISK = "*"
	SLASH    = "/"

	LT   = "<"
	GT   = ">"
	EQEQ = "=="
	NE   = "!="

	COMMA = ","
	SEMI  = ";"

	// Delimiters
	LPAREN   = "("
	RPAREN   = ")"
	LBRACE   = "{"
	RBRACE   = "{"
	LBRACKET = "["
	RBRACKET = "]"

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
var keywords = map[string]Kind{
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
	Kind    Kind
	Literal string
	Span
}

// Span represents a region of code.
type Span struct {
	// Lineno is the line number in the input.
	Lineno int
	// ColumnPos is the postion in the input.
	LineColumn int
}

// NewSpan creates a new span with the line number and position.
func NewSpan(lineno, lineColumn int) Span {
	return Span{Lineno: lineno, LineColumn: lineColumn}
}

// Kind represents the type of a token.
type Kind string

// NewToken create token.
func NewToken(kind Kind, ch rune, span Span) Token {
	return Token{Kind: kind, Literal: string(ch), Span: span}
}

// LookupIdent lookup and identifier in input
func LookupIdent(ident string) Kind {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}

// LookupOperator lookup an operator.
func LookupOp(ch rune) Kind {
	switch ch {
	case '=':
		return EQ
	case '+':
		return PLUS
	case '-':
		return MINUS
	case '!':
		return NOT
	case '*':
		return ASTERISK
	case '/':
		return SLASH
	case '<':
		return LT
	case '>':
		return GT
	default:
		return UNKOWN
	}
}

// LookupDelimiter lookup a delimiter.
func LookupDelimiter(ch rune) Kind {
	switch ch {
	case '(':
		return LPAREN
	case ')':
		return RPAREN
	case '{':
		return LBRACE
	case '}':
		return RBRACE
	case '[':
		return LBRACKET
	case ']':
		return RBRACKET
	default:
		return UNKOWN
	}
}
