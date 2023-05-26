#[macro_export]
/// Defines a token kind variant type.
macro_rules! define_token_kind {
    (
        $kind_name:ident;
        $($name:ident => $literal:literal,)*
    ) => {
        #[derive(Clone, Debug, Copy, PartialEq, Eq)]
        #[doc = concat!(stringify!($kind_name), " type. See module level [documentation](self)")]
        pub enum $kind_name {
            $($name,)*
           Unknown,
        }

        impl $kind_name {
            #[doc = concat!("A slice of literal values for each ", stringify!($kind_name))]
            const LITERALS: &'static[&'static str] = &[$($literal,)*];

            #[doc = concat!("Returns a string slice representing the literal value of the ",
                            stringify!($kind_name))]
            pub const fn as_str(&self) -> &str {
                Self::LITERALS[*self as usize]
            }

        }

        impl std::fmt::Display for $kind_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl From<&str> for $kind_name {
            fn from(value: &str) -> Self {
                match value  {
                    $($literal => Self::$name,)*
                    _ => Self::Unknown
                }

            }

        }

    }
}

define_token_kind! [
    TokenKind;

    Let => "let",
    True => "true",
    False => "false",
    If => "if",
    Else => "else",
    Return => "return",
    Function => "fn",
    Ident => "ident",
    Plus => "+",
    Minus => "-",
    Star => "*",
    Slash => "/",
    Not => "!",
    Eq => "=",
    EqEq => "==",
    Ne => "!=",
    Lt => "<",
    Gt => ">",
    Lparen => "(",
    Rparen => ")",
    Lbrace => "{",
    Rbrace => "}",
    Lbracket => "[",
    Rbracket => "]",
    Comma => ",",
    Semi => ";",
    Number => "number",
    Eof => "",

];
