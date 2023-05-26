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

	checkParserErrors(t, &p)

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
	if s.Literal() != "let" {
		t.Errorf("s.Literal not 'let'. got=%q", s.Literal())
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

	if varDecl.Name.Literal() != name {
		t.Errorf("varDecl.Name.Literal() not '%s'. got=%s", name, varDecl.Name.Literal())
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
