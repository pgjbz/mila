use std::num::{ParseFloatError, ParseIntError};

pub enum ParseError {
    Message(String),
}

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
