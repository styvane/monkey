package lexer

import (
	"github/com/styvane/monkey/token"
	"testing"
)

func TestNextToken(t *testing.T) {
	input := `let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 4;
if (5 < 10) {
return true;
} else {
return false;
}
10 == 10;
10 != 9;
let ∆ = 9;
let śńięg = 9;

`
	tests := []struct {
		expectedKind    token.Kind
		expectedLiteral string
	}{
		{token.LET, "let"},
		{token.IDENT, "five"},
		{token.EQ, "="},
		{token.NUMBER, "5"},
		{token.SEMI, ";"},
		{token.LET, "let"},
		{token.IDENT, "ten"},
		{token.EQ, "="},
		{token.NUMBER, "10"},
		{token.SEMI, ";"},
		{token.LET, "let"},
		{token.IDENT, "add"},
		{token.EQ, "="},
		{token.FUNCTION, "fn"},
		{token.LPAREN, "("},
		{token.IDENT, "x"},
		{token.COMMA, ","},
		{token.IDENT, "y"},
		{token.RPAREN, ")"},
		{token.LBRACE, "{"},
		{token.IDENT, "x"},
		{token.PLUS, "+"},
		{token.IDENT, "y"},
		{token.SEMI, ";"},
		{token.RBRACE, "}"},
		{token.SEMI, ";"},
		{token.LET, "let"},
		{token.IDENT, "result"},
		{token.EQ, "="},
		{token.IDENT, "add"},
		{token.LPAREN, "("},
		{token.IDENT, "five"},
		{token.COMMA, ","},
		{token.IDENT, "ten"},
		{token.RPAREN, ")"},
		{token.SEMI, ";"},
		{token.NOT, "!"},
		{token.MINUS, "-"},
		{token.SLASH, "/"},
		{token.ASTERISK, "*"},
		{token.NUMBER, "5"},
		{token.SEMI, ";"},
		{token.NUMBER, "5"},
		{token.LT, "<"},
		{token.NUMBER, "10"},
		{token.GT, ">"},
		{token.NUMBER, "4"},
		{token.SEMI, ";"},
		{token.IF, "if"},
		{token.LPAREN, "("},
		{token.NUMBER, "5"},
		{token.LT, "<"},
		{token.NUMBER, "10"},
		{token.RPAREN, ")"},
		{token.LBRACE, "{"},
		{token.RETURN, "return"},
		{token.TRUE, "true"},
		{token.SEMI, ";"},
		{token.RBRACE, "}"},
		{token.ELSE, "else"},
		{token.LBRACE, "{"},
		{token.RETURN, "return"},
		{token.FALSE, "false"},
		{token.SEMI, ";"},
		{token.RBRACE, "}"},
		{token.NUMBER, "10"},
		{token.EQEQ, "=="},
		{token.NUMBER, "10"},
		{token.SEMI, ";"},
		{token.NUMBER, "10"},
		{token.NE, "!="},
		{token.NUMBER, "9"},
		{token.SEMI, ";"},
		{token.LET, "let"},
		{token.IDENT, "∆"},
		{token.EQ, "="},
		{token.NUMBER, "9"},
		{token.SEMI, ";"},
		{token.LET, "let"},
		{token.IDENT, "śńięg"},
		{token.EQ, "="},
		{token.NUMBER, "9"},
		{token.SEMI, ";"},
		{token.EOF, ""},
	}

	l := New(input)

	for i, tt := range tests {
		tok := l.NextToken()

		if tok.Kind != tt.expectedKind {
			t.Fatalf("tests[%d] - tokentype wrong. expected=%q, got=%q",
				i, tt.expectedKind, tok.Kind)
		}

		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q, got=%q",
				i, tt.expectedLiteral, tok.Literal)

		}
	}

}
