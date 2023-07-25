package parser

import (
	"testing"

	"github/com/styvane/monkey/ast"
	"github/com/styvane/monkey/lexer"
)

func TestVariableDecl(t *testing.T) {
	input := `
let x = 5;
let y = 10;
let foobar = 838383;
`
	l := lexer.New(input)
	p := New(l)

	program := p.ParseProgram()

	if program == nil {
		t.Fatalf("ParseProgram() return nil")
	}

	checkParserErrors(t, p)

	if len(program.Statements) != 3 {
		t.Fatalf("program.Statements does not contain 3 statements. got=%d", len(program.Statements))
	}

	tests := []struct {
		wantLocalVar string
	}{
		{"x"}, {"y"}, {"foobar"},
	}

	for i, tt := range tests {
		stmt := program.Statements[i]
		if !testLocalVariableDecl(t, stmt, tt.wantLocalVar) {
			return
		}
	}
}

func testLocalVariableDecl(t *testing.T, s ast.Statement, name string) bool {
	if s.TokenLiteral() != "let" {
		t.Errorf("s.Literal not 'let'. got=%q", s.TokenLiteral())
		return false
	}

	varDecl, ok := s.(*ast.LocalVariableDecl)

	if !ok {
		t.Errorf("s not *ast.VariableDecl, got=%T", s)
		return false
	}
	if varDecl.Name.Value != name {
		t.Errorf("varDecl.Name.Value not '%s'.got=%s", name, varDecl.Name.Value)
		return false
	}

	if varDecl.Name.TokenLiteral() != name {
		t.Errorf("varDecl.Name.TokenLiteral() not '%s'. got=%s", name, varDecl.Name.TokenLiteral())
		return false
	}
	return true

}

func checkParserErrors(t *testing.T, p *Parser) {
	errors := p.Errors()
	if len(errors) == 0 {
		return
	}

	t.Errorf("parser has %d errors", len(errors))
	for _, msg := range errors {
		t.Errorf("parser error: %q", msg)
	}

	t.FailNow()
}

func TestInvalidVariableDecl(t *testing.T) {
	input := `
let  = 5;
 y = 10;
let foobar ;
`
	l := lexer.New(input)
	p := New(l)

	program := p.ParseProgram()

	if program == nil {
		t.Fatalf("ParseProgram() return nil")
	}

	if len(p.errors) == 0 {
		t.Errorf("expected parser error")
	}
}

func TestReturnStatement(t *testing.T) {
	input := `
return 5;
return 10;
return 993322;
`

	l := lexer.New(input)
	p := New(l)

	program := p.ParseProgram()
	checkParserErrors(t, p)

	if len(program.Statements) != 3 {
		t.Fatalf("program.Statements does not contains 3 statements. got =%d",
			len(program.Statements))

	}

	for _, stmt := range program.Statements {
		retStmt, ok := stmt.(*ast.ReturnStatement)
		if !ok {
			t.Errorf("stmt not *ast.ReturnStatement.got=%T", stmt)
		}

		if retStmt.TokenLiteral() != "return" {
			t.Errorf("returnStmt.Literal not 'return', got %q", retStmt.TokenLiteral())
		}

	}
}

func TestIdentifierExpr(t *testing.T) {
	input := "foobar;"

	l := lexer.New(input)
	p := New(l)
	program := p.ParseProgram()
	checkParserErrors(t, p)

	if len(program.Statements) != 1 {
		t.Fatalf("program has not enough statements. got=%d", len(program.Statements))
	}

	stmt, ok := program.Statements[0].(*ast.ExpressionStatement)
	if !ok {
		t.Fatalf("program.Statements[0] is not ast.ExpressionStatement. got=%T", program.Statements[0])
	}

	ident, ok := stmt.Expr.(*ast.Identifier)
	if !ok {
		t.Fatalf("expr not *ast.Identifier. got=%T", stmt.Expr)
	}

	if ident.Value != "foobar" {
		t.Errorf("ident.Value not 'foobar'. got=%s", ident.Value)
	}

	if lit := ident.TokenLiteral(); lit != "foobar" {
		t.Errorf("ident.TokenLiteral not 'foobar'. got=%s", lit)
	}
}
