use std::rc::Rc;

use self::token::{location::Location, token_type::TokenType, Token};

pub mod token;

pub struct Lexer {
    source: String,
    current_peek: usize,
    next_peek: usize,
    line: usize,
    line_position: usize,
    current_char: char,
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
            current_char: '\0',
        }
    }

    pub fn next_token(&mut self) -> Token {
        let current_char = self.skip_whitespaces();
        let file = Rc::clone(&self.file);
        let location = Location::new(self.line, self.line_position - 1, file);
        match current_char {
            '.' => Token::new(TokenType::Dot, location, current_char.to_string()),
            ',' => Token::new(TokenType::Comma, location, current_char.to_string()),
            ':' => Token::new(TokenType::Colon, location, current_char.to_string()),
            '+' if self.check_next() == '=' => {
                self.next_char();
                Token::new(TokenType::PlusEq, location, "+=".to_string())
            }
            '+' => Token::new(TokenType::Plus, location, current_char.to_string()),
            '-' if self.check_next() == '=' => {
                self.next_char();
                Token::new(TokenType::MinusEq, location, "-=".to_string())
            }
            '-' => Token::new(TokenType::Minus, location, current_char.to_string()),
            '*' if self.check_next() == '=' => {
                self.next_char();
                Token::new(TokenType::AsteriskEq, location, "*=".to_string())
            }
            '*' => Token::new(TokenType::Asterisk, location, current_char.to_string()),
            '/' if self.check_next() == '=' => {
                self.next_char();
                Token::new(TokenType::SlashEq, location, "/=".to_string())
            }
            '/' if self.check_next() == '/' => {
                self.skip_comment();
                self.next_token()
            }
            '/' => Token::new(TokenType::Slash, location, current_char.to_string()),
            '?' => Token::new(TokenType::Question, location, current_char.to_string()),
            '^' => Token::new(TokenType::Caret, location, current_char.to_string()),
            '&' if self.check_next() == '&' => {
                self.next_char();
                Token::new(TokenType::And, location, "&&".to_string())
            }
            '&' => Token::new(TokenType::BitWiseAnd, location, current_char.to_string()),
            '|' if self.check_next() == '|' => {
                self.next_char();
                Token::new(TokenType::Or, location, "||".to_string())
            }
            '|' => Token::new(TokenType::Pipe, location, current_char.to_string()),
            ';' => Token::new(TokenType::Semicolon, location, current_char.to_string()),
            '{' => Token::new(TokenType::LBrace, location, current_char.to_string()),
            '}' => Token::new(TokenType::RBrace, location, current_char.to_string()),
            '[' => Token::new(TokenType::LBracket, location, current_char.to_string()),
            ']' => Token::new(TokenType::RBracket, location, current_char.to_string()),
            '(' => Token::new(TokenType::LParen, location, current_char.to_string()),
            ')' => Token::new(TokenType::RParen, location, current_char.to_string()),
            '%' => Token::new(TokenType::Mod, location, current_char.to_string()),
            '\0' => Token::new(TokenType::Eof, location, current_char.to_string()),
            '>' if self.check_next() == '=' => {
                self.next_char();
                Token::new(TokenType::GreaterThanOrEq, location, "<=".to_string())
            }
            '>' if self.check_next() == '>' => {
                self.next_char();
                Token::new(TokenType::ShiftRight, location, ">>".to_string())
            }
            '>' => Token::new(TokenType::Greater, location, current_char.to_string()),
            '<' if self.check_next() == '=' => {
                self.next_char();
                Token::new(TokenType::LessThanOrEq, location, "<=".to_string())
            }
            '<' if self.check_next() == '<' => {
                self.next_char();
                Token::new(TokenType::ShiftLeft, location, "<<".to_string())
            }
            '<' => Token::new(TokenType::Less, location, current_char.to_string()),
            '!' if self.check_next() == '=' => {
                self.next_char();
                Token::new(TokenType::NotEq, location, "!=".to_string())
            }
            '!' => Token::new(TokenType::Bang, location, current_char.to_string()),
            '=' if self.check_next() == '=' => {
                self.next_char();
                Token::new(TokenType::Eq, location, "==".to_string())
            }
            '=' => Token::new(TokenType::Assign, location, current_char.to_string()),
            '"' => {
                let string = self.read_string();
                if self.current_char == '"' {
                    self.next_char();
                    Token::new(TokenType::String, location, string)
                } else {
                    Token::new(TokenType::Illegal, location, format!("\"{}", string))
                }
            }
            _ => {
                let value = self.read_char_sequence();
                let word_token = Token::word_token(&value, location.clone());
                if word_token.token_type == TokenType::Illegal {
                    let next_chart = self.current_char;
                    if Self::only_digits(&value) && next_chart != '.' {
                        Token::new(TokenType::Number, location, value)
                    } else if Self::valid_identifier(&value) {
                        Token::new(TokenType::Identifier, location, value)
                    } else if next_chart == '.' {
                        self.next_char();
                        let after_dot = self.next_token();
                        let formated = format!("{}.{}", value, after_dot.value);
                        if after_dot.token_type == TokenType::Number {
                            Token::new(TokenType::FloatingPointNumber, location, formated)
                        } else {
                            Token::new(TokenType::Illegal, location, formated)
                        }
                    } else {
                        Token::new(
                            TokenType::Illegal,
                            location,
                            if value.is_empty() {
                                self.current_char.to_string()
                            } else {
                                value
                            },
                        )
                    }
                } else {
                    word_token
                }
            }
        }
    }

    fn skip_whitespaces(&mut self) -> char {
        let mut current_char = self.next_char();
        while current_char.is_whitespace() {
            current_char = self.next_char();
        }
        current_char
    }

    fn next_char(&mut self) -> char {
        let current_char = self.char_at(self.current_peek);
        if current_char == '\n' {
            self.line += 1;
            self.line_position = 0;
        } else {
            self.line_position += 1;
        }
        self.current_char = current_char;
        self.current_peek = self.next_peek;
        self.next_peek += 1;
        current_char
    }

    fn char_at(&mut self, pos: usize) -> char {
        if pos >= self.source.len() {
            '\0'
        } else {
            self.source[pos..pos + 1].chars().next().unwrap()
        }
    }

    fn read_char_sequence(&mut self) -> String {
        let start_peek = self.current_peek - 1;
        if !Self::is_valid_char(self.current_char) && !self.current_char.is_alphanumeric() {
            while !self.current_char.is_whitespace() && self.current_char != '\0' {
                self.next_char();
            }
        } else {
            while self.current_char.is_alphanumeric() || Self::is_valid_char(self.current_char) {
                self.next_char();
            }
        }
        let final_peek = self.current_peek;
        self.back_peek();
        String::from(&self.source[start_peek..final_peek - 1])
    }

    fn back_peek(&mut self) {
        self.current_peek -= 1;
        self.next_peek -= 1;
        if self.line > 0 && self.line_position == 0 {
            self.line -= 1;
        } else if self.line_position > 0 {
            self.line_position -= 1;
        }
    }

    fn is_valid_char(ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    fn only_digits(value: &str) -> bool {
        if value.is_empty() {
            false
        } else {
            value.chars().all(|c| c.is_numeric())
        }
    }

    fn valid_identifier(value: &str) -> bool {
        if value.is_empty() {
            return false;
        }
        Self::is_valid_char(value.chars().next().unwrap_or('\0'))
    }

    fn check_next(&mut self) -> char {
        self.char_at(self.current_peek)
    }

    fn read_string(&mut self) -> String {
        let start_peek = self.current_peek;
        self.next_char();
        while self.current_char != '"' && self.current_char != '\0' {
            self.next_char();
        }
        let final_peek = self.current_peek;
        self.back_peek();
        String::from(&self.source[start_peek..final_peek - 1])
    }

    fn skip_comment(&mut self) {
        while self.current_char != '\n' && self.current_char != '\0' {
            self.next_char();
        }
    }
}
