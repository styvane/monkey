// Package lexer implements the lexer.
package lexer

import (
	"github/com/styvane/monkey/token"
	"unicode"
)

// Lexer represents the lexer type or tokenizer.
type Lexer struct {
	input        []rune
	lineNumber   int  // current line number in input.
	position     int  // current position in input (points to current char)
	readPosition int  // next character position in input (after current char)
	ch           rune // current char under examination
}

// New returns an initialized Lexer instance.
func New(input string) *Lexer {
	l := &Lexer{input: []rune(input)}
	l.readChar()
	return l
}

// ReadChar reads the next character in the input.
func (l *Lexer) readChar() {
	if l.readPosition >= len(l.input) {
		l.ch = 0
	} else {
		l.ch = l.input[l.readPosition]
	}
	if l.ch == '\n' {
		l.lineNumber += 1
	}
	l.position = l.readPosition
	l.readPosition += 1

}

// NextToken returns the token corresponding to the current input character.
func (l *Lexer) NextToken() token.Token {
	var tok token.Token
	var tokType token.TokenType
	var literal string
	var lineno, position int

	l.skipWhitespace()

	switch l.ch {
	case '=':
		if l.peekChar() == '=' {
			ch := l.ch
			position = l.position
			lineno = l.lineNumber
			l.readChar()
			literal = string(ch) + string(l.ch)
			tokType = token.EQ

		} else {
			tokType = token.ASSIGN
		}

	case ';':
		tokType = token.SEMICOLON
	case '(':
		tokType = token.LPAREN
	case ')':
		tokType = token.RPAREN
	case ',':
		tokType = token.COMMA
	case '+':
		tokType = token.PLUS
	case '{':
		tokType = token.LBRACE
	case '}':
		tokType = token.RBRACE
	case '-':
		tokType = token.MINUS
	case '!':
		if l.peekChar() == '=' {
			ch := l.ch
			position = l.position
			lineno = l.lineNumber
			l.readChar()
			literal = string(ch) + string(l.ch)
			tokType = token.NOT_EQ
		} else {
			tokType = token.BANG
		}
	case '/':
		tokType = token.SLASH
	case '*':
		tokType = token.ASTERISK
	case '<':
		tokType = token.LT
	case '>':
		tokType = token.GT

	case 0:
		tok.Literal = ""
		tok.Type = token.EOF
	default:
		if isLetter(l.ch) {
			position = l.position
			lineno = l.lineNumber
			tok.Literal = l.readIdentifier()
			tok.Type = token.LookupIdent(tok.Literal)
			tok.Span = token.NewSpan(lineno, position)
			return tok
		} else if isDigit(l.ch) {
			position = l.position
			lineno = l.lineNumber
			tok.Type = token.INT
			tok.Literal = l.readNumber()
			tok.Span = token.NewSpan(lineno, position)
			return tok
		} else {
			tokType = token.ILLEGAL
		}
	}

	if literal != "" {
		tok = token.Token{Type: tokType, Literal: literal, Span: token.NewSpan(lineno, position)}
	} else if tok.Type == "" {
		tok = token.NewToken(tokType, l.ch, token.NewSpan(lineno, position))
	}
	l.readChar()
	return tok
}

// readIdentifier reads the next identifier in input.
func (l *Lexer) readIdentifier() string {
	position := l.position
	for isLetter(l.ch) {
		l.readChar()
	}
	return string(l.input[position:l.position])
}

// readNumber reads the next character as number.
func (l *Lexer) readNumber() string {
	position := l.position
	for isDigit(l.ch) {
		l.readChar()
	}
	return string(l.input[position:l.position])
}

// isLetter returns true if the byte corresponds to a letter.
func isLetter(ch rune) bool {
	return unicode.IsLetter(ch) || unicode.IsSymbol(ch) || ch == '_'
}

// Skip white spaces
func (l *Lexer) skipWhitespace() {
	for l.ch == ' ' || l.ch == '\t' || l.ch == '\n' || l.ch == '\r' {
		l.readChar()
	}
}

// isDigit returns true if the byte corresponds to a digit.
func isDigit(ch rune) bool {
	return '0' <= ch && ch <= '9'
}

// Lookahead the next character in input
func (l *Lexer) peekChar() rune {
	if l.readPosition >= len(l.input) {
		return 0
	} else {
		return l.input[l.readPosition]
	}
}
