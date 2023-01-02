// Package lexer implements the lexer.
package lexer

import "github/com/styvane/monkey/token"

type Lexer struct {
	input        string
	position     int  // current position in input (points to current char)
	readPosition int  // next character position in input (after current char)
	ch           byte // current char under examination
}

// New returns an initialized Lexer instance.
func New(input string) *Lexer {
	l := &Lexer{input: input}
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
	l.position = l.readPosition
	l.readPosition += 1

}

// NextToken returns the token corresponding to the current input character.
func (l *Lexer) NextToken() token.Token {
	var tok token.Token
	var tokType token.TokenType
	var literal string

	l.skipWhitespace()

	switch l.ch {
	case '=':
		if l.peekChar() == '=' {
			ch := l.ch
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
			tok.Literal = l.readIdentifier()
			tok.Type = token.LookupIdent(tok.Literal)
			return tok
		} else if isDigit(l.ch) {
			tok.Type = token.INT
			tok.Literal = l.readNumber()
			return tok
		} else {
			tokType = token.ILLEGAL
		}
	}

	if literal != "" {
		tok = token.Token{Type: tokType, Literal: literal}
	} else if tok.Type == "" {
		tok = newToken(tokType, l.ch)
	}
	l.readChar()
	return tok
}

// newToken create token.
func newToken(tokenType token.TokenType, ch byte) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}

// readIdentifier reads the next identifier in input.
func (l *Lexer) readIdentifier() string {
	position := l.position
	for isLetter(l.ch) {
		l.readChar()

	}
	return l.input[position:l.position]
}

// readNumber reads the next character as number.
func (l *Lexer) readNumber() string {
	position := l.position
	for isDigit(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

// isLetter returns true if the byte corresponds to a letter.
func isLetter(ch byte) bool {
	return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

// Skip white spaces
func (l *Lexer) skipWhitespace() {
	for l.ch == ' ' || l.ch == '\t' || l.ch == '\n' || l.ch == '\r' {
		l.readChar()
	}
}

// isDigit returns true if the byte corresponds to a digit.
func isDigit(ch byte) bool {
	return '0' <= ch && ch <= '9'
}

// Lookahead the next character in input
func (l *Lexer) peekChar() byte {
	if l.readPosition >= len(l.input) {
		return 0
	} else {
		return l.input[l.readPosition]
	}
}
