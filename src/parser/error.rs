use std::num::ParseIntError;

pub enum ParseError {
    Message(String),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        Self::Message(e.to_string())
    }
}
