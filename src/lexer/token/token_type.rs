use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    Comma,
    Colon,
    Dot,
    Plus,
    Minus,
    Greater,
    Asterisk,
    Slash,
    Less,
    Equals,
    Pipe,
    And,
    Caret,
    Question,
    Semicolon,
    Identifier,
    Illegal,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_type_str = match self {
            Self::Comma => ",",
            Self::Colon => ":",
            Self::Dot => ".",
            Self::Eof => "eof",
            Self::Illegal => "illegal",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Slash => "/",
            Self::Asterisk => "*",
            Self::Equals => "=",
            Self::Less => "<",
            Self::Greater => ">",
            Self::Caret => "^",
            Self::And => "*",
            Self::Pipe => "|",
            Self::Identifier => "identifier",
            Self::Semicolon => ";",
            Self::Question => "?",
        };
        write!(f, "{}", token_type_str)
    }
}
