use std::rc::Rc;

use mila::lexer::{
    token::{location::Location, token_type::TokenType, Token},
    Lexer,
};

#[test]
fn test_lexer_should_lexer_single_tokens() {
    let source = ":,.+-/* >< =?^&|
;{}()[]!"
        .to_string();
    let filename = Rc::new("tokens.mil".to_string());
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
            TokenType::Assign,
            Location::new(1, 11, Rc::clone(&filename)),
            "=".to_string(),
        ),
        Token::new(
            TokenType::Question,
            Location::new(1, 12, Rc::clone(&filename)),
            "?".to_string(),
        ),
        Token::new(
            TokenType::Caret,
            Location::new(1, 13, Rc::clone(&filename)),
            "^".to_string(),
        ),
        Token::new(
            TokenType::And,
            Location::new(1, 14, Rc::clone(&filename)),
            "&".to_string(),
        ),
        Token::new(
            TokenType::Pipe,
            Location::new(1, 15, Rc::clone(&filename)),
            "|".to_string(),
        ),
        Token::new(
            TokenType::Semicolon,
            Location::new(2, 0, Rc::clone(&filename)),
            ";".to_string(),
        ),
        Token::new(
            TokenType::LBrace,
            Location::new(2, 1, Rc::clone(&filename)),
            "{".to_string(),
        ),
        Token::new(
            TokenType::RBrace,
            Location::new(2, 2, Rc::clone(&filename)),
            "}".to_string(),
        ),
        Token::new(
            TokenType::LParen,
            Location::new(2, 3, Rc::clone(&filename)),
            "(".to_string(),
        ),
        Token::new(
            TokenType::RParen,
            Location::new(2, 4, Rc::clone(&filename)),
            ")".to_string(),
        ),
        Token::new(
            TokenType::LBracket,
            Location::new(2, 5, Rc::clone(&filename)),
            "[".to_string(),
        ),
        Token::new(
            TokenType::RBracket,
            Location::new(2, 6, Rc::clone(&filename)),
            "]".to_string(),
        ),
        Token::new(
            TokenType::Bang,
            Location::new(2, 7, Rc::clone(&filename)),
            "!".to_string(),
        ),
        Token::new(
            TokenType::Eof,
            Location::new(2, 8, Rc::clone(&filename)),
            "\0".to_string(),
        ),
    ];
    test_tokens(lexer, &tokens)
}

#[test]
fn test_lexer_identifier_token() {
    let source = "mila_lang
mila_lang2
_mila_lang mila_lang 
mila.lang
#aaaa"
        .to_string();
    let filename = Rc::new("identififer.mil".to_string());
    let lexer = Lexer::new(source, Rc::clone(&filename));
    let tokens = vec![
        Token::new(
            TokenType::Identifier,
            Location::new(1, 0, Rc::clone(&filename)),
            "mila_lang".to_string(),
        ),
        Token::new(
            TokenType::Identifier,
            Location::new(2, 0, Rc::clone(&filename)),
            "mila_lang2".to_string(),
        ),
        Token::new(
            TokenType::Identifier,
            Location::new(3, 0, Rc::clone(&filename)),
            "_mila_lang".to_string(),
        ),
        Token::new(
            TokenType::Identifier,
            Location::new(3, 11, Rc::clone(&filename)),
            "mila_lang".to_string(),
        ),
        Token::new(
            TokenType::Identifier,
            Location::new(4, 0, Rc::clone(&filename)),
            "mila".to_string(),
        ),
        Token::new(
            TokenType::Dot,
            Location::new(4, 4, Rc::clone(&filename)),
            ".".to_string(),
        ),
        Token::new(
            TokenType::Identifier,
            Location::new(4, 5, Rc::clone(&filename)),
            "lang".to_string(),
        ),
        Token::new(
            TokenType::Illegal,
            Location::new(5, 0, Rc::clone(&filename)),
            "#aaaa".to_string(),
        ),
    ];
    test_tokens(lexer, &tokens)
}

#[test]
fn test_lexer_number_token() {
    let source = "12345;123456
123456789>
123456<458444|
121221&
125478^
1,2
1477a;"
        .to_string();
    let filename = Rc::new("number.mil".to_string());
    let lexer = Lexer::new(source, Rc::clone(&filename));
    let tokens = vec![
        Token::new(
            TokenType::Number,
            Location::new(1, 0, Rc::clone(&filename)),
            "12345".to_string(),
        ),
        Token::new(
            TokenType::Semicolon,
            Location::new(1, 5, Rc::clone(&filename)),
            ";".to_string(),
        ),
        Token::new(
            TokenType::Number,
            Location::new(1, 6, Rc::clone(&filename)),
            "123456".to_string(),
        ),
        Token::new(
            TokenType::Number,
            Location::new(2, 0, Rc::clone(&filename)),
            "123456789".to_string(),
        ),
        Token::new(
            TokenType::Greater,
            Location::new(2, 9, Rc::clone(&filename)),
            ">".to_string(),
        ),
        Token::new(
            TokenType::Number,
            Location::new(3, 0, Rc::clone(&filename)),
            "123456".to_string(),
        ),
        Token::new(
            TokenType::Less,
            Location::new(3, 6, Rc::clone(&filename)),
            "<".to_string(),
        ),
        Token::new(
            TokenType::Number,
            Location::new(3, 7, Rc::clone(&filename)),
            "458444".to_string(),
        ),
        Token::new(
            TokenType::Pipe,
            Location::new(3, 13, Rc::clone(&filename)),
            "|".to_string(),
        ),
        Token::new(
            TokenType::Number,
            Location::new(4, 0, Rc::clone(&filename)),
            "121221".to_string(),
        ),
        Token::new(
            TokenType::And,
            Location::new(4, 6, Rc::clone(&filename)),
            "&".to_string(),
        ),
        Token::new(
            TokenType::Number,
            Location::new(5, 0, Rc::clone(&filename)),
            "125478".to_string(),
        ),
        Token::new(
            TokenType::Caret,
            Location::new(5, 6, Rc::clone(&filename)),
            "^".to_string(),
        ),
        Token::new(
            TokenType::Number,
            Location::new(6, 0, Rc::clone(&filename)),
            "1".to_string(),
        ),
        Token::new(
            TokenType::Comma,
            Location::new(6, 1, Rc::clone(&filename)),
            ",".to_string(),
        ),
        Token::new(
            TokenType::Number,
            Location::new(6, 2, Rc::clone(&filename)),
            "2".to_string(),
        ),
        Token::new(
            TokenType::Illegal,
            Location::new(7, 0, Rc::clone(&filename)),
            "1477a".to_string(),
        ),
        Token::new(
            TokenType::Semicolon,
            Location::new(7, 5, Rc::clone(&filename)),
            ";".to_string(),
        ),
    ];
    test_tokens(lexer, &tokens);
}

#[test]
fn test_lexer_floating_pointer_token() {
    let source = "100.0
100.0;
100.0a
100.1
100.0.0"
        .to_string();
    let filename = Rc::new("floating.mil".to_string());
    let lexer = Lexer::new(source, Rc::clone(&filename));
    let tokens = vec![
        Token::new(
            TokenType::FloatingPointNumber,
            Location::new(1, 0, Rc::clone(&filename)),
            "100.0".to_string(),
        ),
        Token::new(
            TokenType::FloatingPointNumber,
            Location::new(2, 0, Rc::clone(&filename)),
            "100.0".to_string(),
        ),
        Token::new(
            TokenType::Semicolon,
            Location::new(2, 5, Rc::clone(&filename)),
            ";".to_string(),
        ),
        Token::new(
            TokenType::Illegal,
            Location::new(3, 0, Rc::clone(&filename)),
            "100.0a".to_string(),
        ),
        Token::new(
            TokenType::FloatingPointNumber,
            Location::new(4, 0, Rc::clone(&filename)),
            "100.1".to_string(),
        ),
        Token::new(
            TokenType::Illegal,
            Location::new(5, 0, Rc::clone(&filename)),
            "100.0.0".to_string(),
        ),
    ];
    test_tokens(lexer, &tokens);
}

#[test]
fn test_word_token() {
    let source = "let var while true false fn ret".to_string();
    let filename = Rc::new("word_token.mil".to_string());
    let lexer = Lexer::new(source, Rc::clone(&filename));
    let tokens = vec![
        Token::new(
            TokenType::Let,
            Location::new(1, 0, Rc::clone(&filename)),
            "let".to_string(),
        ),
        Token::new(
            TokenType::Var,
            Location::new(1, 4, Rc::clone(&filename)),
            "var".to_string(),
        ),
        Token::new(
            TokenType::While,
            Location::new(1, 8, Rc::clone(&filename)),
            "while".to_string(),
        ),
        Token::new(
            TokenType::True,
            Location::new(1, 14, Rc::clone(&filename)),
            "true".to_string(),
        ),
        Token::new(
            TokenType::False,
            Location::new(1, 19, Rc::clone(&filename)),
            "false".to_string(),
        ),
        Token::new(
            TokenType::Fn,
            Location::new(1, 25, Rc::clone(&filename)),
            "fn".to_string(),
        ),
        Token::new(
            TokenType::Ret,
            Location::new(1, 28, Rc::clone(&filename)),
            "ret".to_string(),
        ),
    ];
    test_tokens(lexer, &tokens);
}
#[test]
fn test_two_char_token() {
    let source = "== != <= >= >> << += -= *= /=".to_string();
    let filename = Rc::new("two_char.mil".to_string());
    let lexer = Lexer::new(source, Rc::clone(&filename));
    let tokens = vec![
        Token::new(
            TokenType::Eq,
            Location::new(1, 0, Rc::clone(&filename)),
            "==".to_string(),
        ),
        Token::new(
            TokenType::NotEq,
            Location::new(1, 3, Rc::clone(&filename)),
            "!=".to_string(),
        ),
        Token::new(
            TokenType::LessThanOrEq,
            Location::new(1, 6, Rc::clone(&filename)),
            "<=".to_string(),
        ),
        Token::new(
            TokenType::GreaterThanOrEq,
            Location::new(1, 9, Rc::clone(&filename)),
            "<=".to_string(),
        ),
        Token::new(
            TokenType::ShiftRight,
            Location::new(1, 12, Rc::clone(&filename)),
            ">>".to_string(),
        ),
        Token::new(
            TokenType::ShiftLeft,
            Location::new(1, 15, Rc::clone(&filename)),
            "<<".to_string(),
        ),
        Token::new(
            TokenType::PlusEq,
            Location::new(1, 18, Rc::clone(&filename)),
            "+=".to_string(),
        ),
        Token::new(
            TokenType::MinusEq,
            Location::new(1, 21, Rc::clone(&filename)),
            "-=".to_string(),
        ),
        Token::new(
            TokenType::AsteriskEq,
            Location::new(1, 24, Rc::clone(&filename)),
            "*=".to_string(),
        ),
        Token::new(
            TokenType::SlashEq,
            Location::new(1, 27, Rc::clone(&filename)),
            "/=".to_string(),
        ),
    ];
    test_tokens(lexer, &tokens);
}

#[test]
fn test_string_token() {
    let source = "\"mila_lang\"
\"banana
\";
\"aaaaaa"
        .to_string();
    let filename = Rc::new("string.mil".to_string());
    let lexer = Lexer::new(source, Rc::clone(&filename));
    let tokens = vec![
        Token::new(
            TokenType::String,
            Location::new(1, 0, Rc::clone(&filename)),
            "mila_lang".to_string(),
        ),
        Token::new(
            TokenType::String,
            Location::new(2, 0, Rc::clone(&filename)),
            "banana\n".to_string(),
        ),
        Token::new(
            TokenType::Semicolon,
            Location::new(3, 1, Rc::clone(&filename)),
            ";".to_string(),
        ),
        Token::new(
            TokenType::Illegal,
            Location::new(4, 0, Rc::clone(&filename)),
            "\"aaaaaa".to_string(),
        ),
    ];
    test_tokens(lexer, &tokens);
}

fn test_tokens(mut lexer: Lexer, tokens: &[Token]) {
    for token in tokens {
        assert_eq!(*token, lexer.next_token())
    }
}
