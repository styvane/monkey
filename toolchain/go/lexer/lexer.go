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
	l.lineNumber += 1
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

// isDelimiter returns true if the character is a delimiter.
func isDelimiter(ch rune) bool {
	switch ch {
	case '(', ')', '{', '}', '[', ']':
		return true
	default:
		return false
	}
}

// isOp returns true if the character is an operator.
func isOp(ch rune) bool {
	switch ch {
	case '+', '-', '*', '/', '!', '=', '<', '>':
		return true
	default:
		return false
	}
}

// NextToken returns the current input character's token.
func (l *Lexer) NextToken() token.Token {
	var tok token.Token
	var tokKind token.Kind
	var literal string
	l.eatWhitespace()
	position := l.position
	lineno := l.lineNumber

	switch {
	case l.ch == ';':
		tokKind = token.SEMI
	case isDelimiter(l.ch):
		tokKind = token.LookupDelimiter(l.ch)
	case l.ch == ',':
		tokKind = token.COMMA
	case isOp(l.ch):
		if l.ch == '!' && l.peekChar() == '=' {
			ch := l.ch
			l.readChar()
			literal = string(ch) + string(l.ch)
			tokKind = token.NE
		} else if l.ch == '=' && l.peekChar() == '=' {
			ch := l.ch
			l.readChar()
			literal = string(ch) + string(l.ch)
			tokKind = token.EQEQ

		} else {
			tokKind = token.LookupOp(l.ch)
		}

	case l.ch == 0:
		tok.Literal = ""
		tok.Kind = token.EOF
	default:
		if isLetter(l.ch) {
			tok.Literal = l.readIdentifier()
			tok.Kind = token.LookupIdent(tok.Literal)
			tok.Span = token.NewSpan(lineno, position)
			return tok
		} else if isDigit(l.ch) {
			position = l.position
			lineno = l.lineNumber
			tok.Kind = token.NUMBER
			tok.Literal = l.readNumber()
			tok.Span = token.NewSpan(lineno, position)
			return tok
		} else {
			tokKind = token.UNKOWN
		}
	}

	if literal != "" {
		tok = token.Token{Kind: tokKind, Literal: literal, Span: token.NewSpan(lineno, position)}
	} else if tok.Kind == "" {
		tok = token.NewToken(tokKind, string(l.ch), token.NewSpan(l.lineNumber, position))
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

// eatWhitespace skips white spaces
func (l *Lexer) eatWhitespace() {
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
