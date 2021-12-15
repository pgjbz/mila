use std::fmt::Display;

pub enum TokenType {
    Comma,
    Colon,
    Dot,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_type_str = match self {
            Self::Comma => ",",
            Self::Colon => ":",
            Self::Dot => ".",  
        };
        write!(f, "{}", token_type_str)
    }
}