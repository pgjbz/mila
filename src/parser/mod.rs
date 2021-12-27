mod error;
mod parse_prefix_fns;
mod precedence;

use std::collections::HashMap;

use crate::{
    ast::{
        node::{statements::expression_stmt::ExpressionStmt, Node},
        Program,
    },
    lexer::{
        token::{token_type::TokenType, Token},
        Lexer,
    },
};

use self::{error::ParseError, precedence::Precedence};

pub type ParsePrefixFn = fn(&mut Parser) -> Result<Box<dyn Node>, ParseError>;

pub struct Parser {
    lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
    pub parse_prefix_fns: HashMap<TokenType, ParsePrefixFn>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let mut parse_prefix_fns: HashMap<TokenType, ParsePrefixFn> = HashMap::new();
        parse_prefix_fns.insert(TokenType::Bang, parse_prefix_fns::parse_prefix_expr);
        parse_prefix_fns.insert(TokenType::Minus, parse_prefix_fns::parse_prefix_expr);
        parse_prefix_fns.insert(TokenType::True, parse_prefix_fns::parse_boolean_expr);
        parse_prefix_fns.insert(TokenType::False, parse_prefix_fns::parse_boolean_expr);
        parse_prefix_fns.insert(TokenType::Number, parse_prefix_fns::parse_int_expr);
        parse_prefix_fns.insert(TokenType::String, parse_prefix_fns::parse_string_expr);
        parse_prefix_fns.insert(TokenType::Identifier, parse_prefix_fns::parse_string_expr);
        parse_prefix_fns.insert(
            TokenType::FloatingPointNumber,
            parse_prefix_fns::parse_float_expr,
        );
        Self {
            current_token,
            peek_token,
            lexer,
            parse_prefix_fns,
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        loop {
            match self.parse_statement() {
                Ok(stmt) => program.push_statements(stmt),
                Err(ParseError::Message(e)) => program.push_error(e),
            }
            self.next_token();
            if self.current_token.token_type == TokenType::Eof {
                break;
            }
        }
        program
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Node>, ParseError> {
        match self.current_token.token_type {
            //for now use match only for... nothing, but this match expr will make difference
            _ => self.parse_expr_estatement(),
        }
    }

    fn parse_expr_estatement(&mut self) -> Result<Box<dyn Node>, ParseError> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Ok(Box::new(ExpressionStmt::new(expr)))
    }

    fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        token_type == self.current_token.token_type
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Result<Box<dyn Node>, ParseError> {
        let current_token_type = self.current_token.token_type;
        let left_expr = match self.parse_prefix_fns.get(&current_token_type) {
            Some(function) => function(self)?,
            None => {
                let msg = format!(
                    "error, expected prefix like '!' or '-', got {}",
                    self.current_token
                );
                return Err(ParseError::Message(msg));
            }
        };
        Ok(left_expr)
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }
}
