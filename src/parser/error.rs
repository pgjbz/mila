use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum ParseError {
    Message(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Message(msg) => msg,
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        Self::Message(e.to_string())
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(e: ParseFloatError) -> Self {
        Self::Message(e.to_string())
    }
}
