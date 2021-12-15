use std::rc::Rc;

use self::token::{location::Location, token_type::TokenType, Token};

pub mod token;

pub struct Lexer {
    source: String,
    current_peek: usize,
    next_peek: usize,
    line: usize,
    line_position: usize,
    file: Rc<String>,
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
        } else {
            self.line_position += 1;
        }
        self.current_peek = self.next_peek;
        self.next_peek += 1;
        result
    }

    fn skip_whitespaces(&mut self) -> char {
        let mut current_char = self.next_char();
        while current_char.is_whitespace() {
            current_char = self.next_char();
        }
        current_char
    }

    pub fn next_token(&mut self) -> Token {
        let current_char = self.skip_whitespaces();
        let file = Rc::clone(&self.file);
        let location = Location::new(self.line, self.line_position - 1, file);
        match current_char {
            '.' => Token::new(TokenType::Dot, location, current_char.to_string()),
            ',' => Token::new(TokenType::Comma, location, current_char.to_string()),
            ':' => Token::new(TokenType::Colon, location, current_char.to_string()),
            '+' => Token::new(TokenType::Plus, location, current_char.to_string()),
            '-' => Token::new(TokenType::Minus, location, current_char.to_string()),
            '*' => Token::new(TokenType::Asterisk, location, current_char.to_string()),
            '/' => Token::new(TokenType::Slash, location, current_char.to_string()),
            '>' => Token::new(TokenType::Greater, location, current_char.to_string()),
            '<' => Token::new(TokenType::Less, location, current_char.to_string()),
            '?' => Token::new(TokenType::Question, location, current_char.to_string()),
            '=' => Token::new(TokenType::Equals, location, current_char.to_string()),
            '^' => Token::new(TokenType::Caret, location, current_char.to_string()),
            '&' => Token::new(TokenType::And, location, current_char.to_string()),
            '|' => Token::new(TokenType::Pipe, location, current_char.to_string()),
            ';' => Token::new(TokenType::Semicolon, location, current_char.to_string()),
            '\0' => Token::new(TokenType::Eof, location, current_char.to_string()),
            _ => Token::new(TokenType::Illegal, location, current_char.to_string()),
        }
    }
}
