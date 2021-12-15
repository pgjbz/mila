use std::rc::Rc;

use mila::lexer::{Lexer, token::{token_type::TokenType, Token, location::Location}};

#[test]
fn test_lexer_should_lexer_single_tokens() {
    let source = ":,.".to_string();
    let filename = Rc::new("test.mil".to_string());
    let lexer = Lexer::new(source, Rc::clone(&filename));
    let tokens = vec![
        Token::new(TokenType::Colon, Location::new(1, 1, Rc::clone(&filename)), ":".to_string()),
        Token::new(TokenType::Comma, Location::new(1, 2, Rc::clone(&filename)), ",".to_string()),
        Token::new(TokenType::Dot, Location::new(1, 3, Rc::clone(&filename)), ".".to_string()),
        Token::new(TokenType::Eof, Location::new(1, 4, Rc::clone(&filename)), "\0".to_string()),
    ];
    test_tokens(lexer, &tokens)
}

fn test_tokens(mut lexer: Lexer, tokens: &[Token]) {
    for token in tokens {
        assert_eq!(*token, lexer.next_token())
    }
}