use std::rc::Rc;

use mila::lexer::{
    token::{location::Location, token_type::TokenType, Token},
    Lexer,
};

#[test]
fn test_lexer_should_lexer_single_tokens() {
    let source = ":,.+-/* ><=?^&|
;"
    .to_string();
    let filename = Rc::new("test.mil".to_string());
    let lexer = Lexer::new(source, Rc::clone(&filename));
    let tokens = vec![
        Token::new(
            TokenType::Colon,
            Location::new(1, 0, Rc::clone(&filename)),
            ":".to_string(),
        ),
        Token::new(
            TokenType::Comma,
            Location::new(1, 1, Rc::clone(&filename)),
            ",".to_string(),
        ),
        Token::new(
            TokenType::Dot,
            Location::new(1, 2, Rc::clone(&filename)),
            ".".to_string(),
        ),
        Token::new(
            TokenType::Plus,
            Location::new(1, 3, Rc::clone(&filename)),
            "+".to_string(),
        ),
        Token::new(
            TokenType::Minus,
            Location::new(1, 4, Rc::clone(&filename)),
            "-".to_string(),
        ),
        Token::new(
            TokenType::Slash,
            Location::new(1, 5, Rc::clone(&filename)),
            "/".to_string(),
        ),
        Token::new(
            TokenType::Asterisk,
            Location::new(1, 6, Rc::clone(&filename)),
            "*".to_string(),
        ),
        Token::new(
            TokenType::Greater,
            Location::new(1, 8, Rc::clone(&filename)),
            ">".to_string(),
        ),
        Token::new(
            TokenType::Less,
            Location::new(1, 9, Rc::clone(&filename)),
            "<".to_string(),
        ),
        Token::new(
            TokenType::Equals,
            Location::new(1, 10, Rc::clone(&filename)),
            "=".to_string(),
        ),
        Token::new(
            TokenType::Question,
            Location::new(1, 11, Rc::clone(&filename)),
            "?".to_string(),
        ),
        Token::new(
            TokenType::Caret,
            Location::new(1, 12, Rc::clone(&filename)),
            "^".to_string(),
        ),
        Token::new(
            TokenType::And,
            Location::new(1, 13, Rc::clone(&filename)),
            "&".to_string(),
        ),
        Token::new(
            TokenType::Pipe,
            Location::new(1, 14, Rc::clone(&filename)),
            "|".to_string(),
        ),
        Token::new(
            TokenType::Semicolon,
            Location::new(2, 0, Rc::clone(&filename)),
            ";".to_string(),
        ),
        Token::new(
            TokenType::Eof,
            Location::new(2, 1, Rc::clone(&filename)),
            "\0".to_string(),
        ),
    ];
    test_tokens(lexer, &tokens)
}

fn test_tokens(mut lexer: Lexer, tokens: &[Token]) {
    for token in tokens {
        assert_eq!(*token, lexer.next_token())
    }
}
