use std::rc::Rc;

use mila::lexer::token::{location::Location, token_type::TokenType, Token};

fn main() {
    let token = Token::new(
        TokenType::Comma,
        Location::new(1, 2, Rc::new("foo.mil".to_string())),
    );
    println!("{}", token)
}
