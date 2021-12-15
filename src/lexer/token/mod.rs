use std::fmt::Display;

use self::{location::Location, token_type::TokenType};
pub mod location;
pub mod token_type;

pub struct Token {
    pub token_type: TokenType,
    pub location: Location,
}

impl Token {
    pub fn new(token_type: TokenType, location: Location) -> Self {
        Self {
            token_type,
            location,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' in {}", self.token_type, self.location)
    }
}