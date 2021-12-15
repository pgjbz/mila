use std::rc::Rc;

use self::token::{Token, location::Location, token_type::TokenType};

pub mod token;

pub struct Lexer {
    source: String,
    current_peek: usize,
    next_peek: usize,
    line: usize,
    line_position: usize,
    file: Rc<String>
}

impl Lexer {
    pub fn new(source: String, file: Rc<String>) -> Self {
        Self {
            source,
            current_peek: 0,
            next_peek: 1,
            line: 1,
            line_position: 0,
            file,
        }
    }

    fn next_char(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current_peek);
        let result = if let Some(current_char) = current_char {
            current_char
        } else {
            '\0'
        };
        if result == '\n' {
            self.line += 1;
            self.line_position = 0;
        }
        self.line_position += 1;
        self.current_peek = self.next_peek;
        self.next_peek += 1;
        result
    }

    pub fn next_token(&mut self) -> Token {
        let current_char = self.next_char();
        let file = Rc::clone(&self.file);
        let location = Location::new(self.line, self.line_position, file);
        match current_char {
            '.' => Token::new(TokenType::Dot, location, current_char.to_string()),
            ',' => Token::new(TokenType::Comma, location, current_char.to_string()),
            ':' => Token::new(TokenType::Colon, location, current_char.to_string()),
            '\0' => Token::new(TokenType::Eof, location, current_char.to_string()),
            _ => Token::new(TokenType::Illegal, location, current_char.to_string())
        }
    }
}