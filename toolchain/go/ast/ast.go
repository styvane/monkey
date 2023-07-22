// Package ast implement the AST data structures.
package ast

import (
	"fmt"
	"github/com/styvane/monkey/token"
	"strings"
)

// A Node provide a literal value of the token it's associate
// with.
type Node interface {
	fmt.Stringer
	TokenLiteral() string
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

func (p *Program) TokenLiteral() string {
	if len(p.Statements) > 0 {
		return p.Statements[0].TokenLiteral()
	}
	return ""
}

func (p *Program) String() string {
	var out strings.Builder
	for _, s := range p.Statements {
		fmt.Fprint(&out, s.String())
	}
	return out.String()
}

// LocalVariableDecl represents a variable declaration.
type LocalVariableDecl struct {
	Token token.Token // the token.LET token.
	Name  *Identifier
	Value Expression
}

func (lv *LocalVariableDecl) statementNode()       {}
func (lv *LocalVariableDecl) TokenLiteral() string { return lv.Token.Literal }

func (lv *LocalVariableDecl) String() string {
	var out strings.Builder
	fmt.Fprintf(&out, "%s %s = ", lv.TokenLiteral(), lv.Name.String())
	if lv.Value != nil {
		out.WriteString(lv.Value.TokenLiteral())
	}
	out.WriteString(";")
	return out.String()
}

// Identifier represents an identifier's name.
type Identifier struct {
	Token token.Token // the token.IDENT token.
	Value string
}

func (i *Identifier) TokenLiteral() string { return i.Token.Literal }

func (i *Identifier) String() string {
	return i.Value
}
func (i *Identifier) expressionNode() {}

// ReturnStatement represents a return statement.
type ReturnStatement struct {
	Token token.Token
	Value Expression
}

func (rs *ReturnStatement) statementNode()       {}
func (rs *ReturnStatement) TokenLiteral() string { return rs.Token.Literal }

func (rs *ReturnStatement) String() string {
	var out strings.Builder
	fmt.Fprintf(&out, "%s ", rs.TokenLiteral())
	if rs.Value != nil {
		out.WriteString(rs.Value.String())
	}

	out.WriteString(";")

	return out.String()
}

// ExpressionStatement wraps an expression.
type ExpressionStatement struct {
	Token token.Token // First token in the associated expression.
	Expr  Expression
}

func (es *ExpressionStatement) statementNode() {}

func (es *ExpressionStatement) TokenLiteral() string {
	return es.Token.Literal
}

func (es *ExpressionStatement) String() string {
	if es != nil {
		return es.Expr.String()
	}
	return ""
}
