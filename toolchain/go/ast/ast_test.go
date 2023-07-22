package ast

import (
	"github/com/styvane/monkey/token"
	"testing"
)

func TestString(t *testing.T) {
	program := &Program{
		Statements: []Statement{
			&LocalVariableDecl{
				Token: token.NewToken(token.LET, "let", token.Span{}),
				Name: &Identifier{
					Token: token.NewToken(token.IDENT, "myVar", token.Span{}),
					Value: "myVar",
				},
				Value: &Identifier{
					Token: token.NewToken(token.IDENT, "anotherVar", token.Span{}),
				},
			},
		},
	}

	if str := program.String(); str != "let myVar = anotherVar;" {
		t.Errorf("program.String() wrong, got=%q", str)
	}
}
