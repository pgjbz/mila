use std::fmt::Display;

pub use self::{location::Location, token_type::TokenType};

mod location;
mod token_type;

pub use location::*;
pub use token_type::*;

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
            "if" => Token::new(TokenType::If, location, word.to_string()),
            "var" => Token::new(TokenType::Var, location, word.to_string()),
            "while" => Token::new(TokenType::While, location, word.to_string()),
            "true" => Token::new(TokenType::True, location, word.to_string()),
            "false" => Token::new(TokenType::False, location, word.to_string()),
            "ret" => Token::new(TokenType::Ret, location, word.to_string()),
            "fn" => Token::new(TokenType::Fn, location, word.to_string()),
            "else" => Token::new(TokenType::Else, location, word.to_string()),
            _ => Token::new(TokenType::Illegal, location, word.to_string()),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' in {}", self.value, self.location)
    }
}
