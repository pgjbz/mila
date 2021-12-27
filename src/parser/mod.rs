mod error;
mod infix_fns;
mod precedence;
mod prefix_fns;

use crate::precedence;
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
use std::collections::HashMap;

use self::{error::ParseError, precedence::Precedence};

pub type ParsePrefixFn = fn(&mut Parser) -> ParseResult;
pub type ParseInfixFn = fn(&mut Parser, Box<dyn Node>) -> ParseResult;
pub type ParseResult = Result<Box<dyn Node>, ParseError>;

pub struct Parser {
    lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
    pub parse_prefix_fns: HashMap<TokenType, ParsePrefixFn>,
    pub parse_infix_fns: HashMap<TokenType, ParseInfixFn>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let mut parse_prefix_fns: HashMap<TokenType, ParsePrefixFn> = HashMap::new();
        let mut parse_infix_fns: HashMap<TokenType, ParseInfixFn> = HashMap::new();
        parse_prefix_fns.insert(TokenType::Bang, prefix_fns::parse_prefix_expr);
        parse_prefix_fns.insert(TokenType::Minus, prefix_fns::parse_prefix_expr);
        parse_prefix_fns.insert(TokenType::True, prefix_fns::parse_boolean_expr);
        parse_prefix_fns.insert(TokenType::False, prefix_fns::parse_boolean_expr);
        parse_prefix_fns.insert(TokenType::Number, prefix_fns::parse_int_expr);
        parse_prefix_fns.insert(TokenType::String, prefix_fns::parse_string_expr);
        parse_prefix_fns.insert(TokenType::Identifier, prefix_fns::parse_identifier_expr);
        parse_prefix_fns.insert(TokenType::FloatingPointNumber, prefix_fns::parse_float_expr);

        parse_infix_fns.insert(TokenType::Plus, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::PlusAssign, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Minus, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::MinusAssign, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Asterisk, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::AsteriskAssign, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Slash, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::SlashAssign, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Pipe, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Caret, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::BitWiseAnd, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Mod, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Or, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::And, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::LessThanOrEq, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Less, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Greater, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(
            TokenType::GreaterThanOrEq,
            infix_fns::parse_infix_expression,
        );
        parse_infix_fns.insert(TokenType::Assign, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::NotEq, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::Eq, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::ShiftLeft, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::ShiftRight, infix_fns::parse_infix_expression);
        Self {
            current_token,
            peek_token,
            lexer,
            parse_prefix_fns,
            parse_infix_fns,
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

    fn parse_statement(&mut self) -> ParseResult {
        match self.current_token.token_type {
            //for now use match only for... nothing, but this match expr will make difference
            _ => self.parse_expr_estatement(),
        }
    }

    fn parse_expr_estatement(&mut self) -> ParseResult {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Ok(Box::new(ExpressionStmt::new(expr)))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ParseResult {
        let current_token_type = self.current_token.token_type;
        let mut left_expr = match self.parse_prefix_fns.get(&current_token_type) {
            Some(function) => function(self)?,
            None => {
                let msg = format!(
                    "error, expected prefix like '!' or '-', got {}",
                    self.current_token
                );
                return Err(ParseError::Message(msg));
            }
        };

        while !self.current_token_is(TokenType::Semicolon)
            && precedence < precedence!(self.peek_token.token_type)
        {
            let infix = if let Some(infix) = self.parse_infix_fns.get(&self.peek_token.token_type) {
                infix
            } else {
                return Ok(left_expr);
            };
            left_expr = infix(self, left_expr)?;
        }
        Ok(left_expr)
    }

    fn current_token_is(&mut self, token_type: TokenType) -> bool {
        token_type == self.current_token.token_type
    }

    fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        token_type == self.peek_token.token_type
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }
}
