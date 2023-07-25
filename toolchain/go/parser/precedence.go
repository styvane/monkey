package parser

const (
	_ int = iota
	LOWEST
	EQUALS      // ==
	LESSGREATER // > or <
	SUM         // +
	PRODUCT     // +
	PREFIX      // -x or !x
	CALL        // someFunc(x)
)
