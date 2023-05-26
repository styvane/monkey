package parser

import (
	"fmt"
	"github/com/styvane/monkey/token"
)

// / ParseError is an error encountered during parsing.
type ParseError struct {
	expected, found token.Kind
}

func error(p *ParseError) string {
	return fmt.Sprintf("expected token to be %q, got %q instead", p.expected, p.found)
}
