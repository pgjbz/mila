use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    Comma,
    Colon,
    Dot,
    Eof,
    Illegal,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_type_str = match self {
            Self::Comma => ",",
            Self::Colon => ":",
            Self::Dot => ".",  
            Self::Eof => "eof",
            Self::Illegal => "illegal",
        };
        write!(f, "{}", token_type_str)
    }
}