use std::fmt::Display;

use self::{location::Location, token_type::TokenType};
pub mod location;
pub mod token_type;

#[derive(PartialEq, Eq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub location: Location,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, location: Location, value: String) -> Self {
        Self {
            token_type,
            location,
            value,
        }
    }
}

impl Token {
    pub(super) fn word_token(word: &str, location: Location) -> Token {
        match word {
            "let" => Token::new(TokenType::Let, location, word.to_string()),
            _ => Token::new(TokenType::Illegal, location, word.to_string()),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' in {}", self.value, self.location)
    }
}
