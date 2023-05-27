// Package ast implement the AST data structures.
package ast

import "github/com/styvane/monkey/token"

// A Node provide a literal value of the token it's associate
// with.
type Node interface {
	Literal() string
}

// The  Statement interface is implemented by the nodes that are
// statement.
type Statement interface {
	Node
	statementNode()
}

// The Expression interface is implemented by nodes that are
// expression.
type Expression interface {
	Node
	expressionNode()
}

// Program is the program's root node.
type Program struct {
	// A list of statements that make the program.
	Statements []Statement
}

func (p *Program) Literal() string {
	if len(p.Statements) > 0 {
		return p.Statements[0].Literal()
	}
	return ""
}

// LocalVariableDecl represents a variable declaration.
type LocalVariableDecl struct {
	Name  *VarName
	Token token.Token // the token.LET token.
	Value Expression
}

func (v *LocalVariableDecl) statementNode()  {}
func (v *LocalVariableDecl) Literal() string { return v.Token.Literal }

// VarName represents an identifier's name.
type VarName struct {
	Token token.Token // the token.IDENT token.
	Value string
}

func (v *VarName) Literal() string { return v.Token.Literal }

// ReturnStatement represents a return statement.
type ReturnStatement struct {
	Token token.Token
	Value Expression
}

func (r *ReturnStatement) statementNode()  {}
func (r *ReturnStatement) Literal() string { return r.Token.Literal }
