mod error;
mod infix_fns;
mod precedence;
mod prefix_fns;

use crate::ast::node::LetStatement;
use crate::ast::node::NodeRef;
use crate::ast::node::RetStatement;
use crate::ast::node::VarStatement;
use crate::precedence;
use crate::{
    ast::{node::ExpressionStmt, Program},
    lexer::{Lexer, Token, TokenType},
};
use std::collections::HashMap;

use self::{error::ParseError, precedence::Precedence};

pub type ParsePrefixFn = fn(&mut Parser) -> ParseResult;
pub type ParseInfixFn = fn(&mut Parser, NodeRef) -> ParseResult;
pub type ParseResult = Result<NodeRef, ParseError>;

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
        parse_prefix_fns.insert(TokenType::LParen, prefix_fns::parse_group_expr);
        // parse_prefix_fns.insert(TokenType::LBrace, prefix_fns::parse_block_stmt);
        parse_prefix_fns.insert(TokenType::If, prefix_fns::parse_if_expr);
        parse_prefix_fns.insert(TokenType::While, prefix_fns::parse_while_expr);
        parse_prefix_fns.insert(TokenType::Fn, prefix_fns::parse_fn_expr);
        parse_prefix_fns.insert(TokenType::LBracket, prefix_fns::parse_array_expr);
        parse_prefix_fns.insert(TokenType::Pipe, prefix_fns::parse_hash_expr);

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
        parse_infix_fns.insert(TokenType::Dot, infix_fns::parse_infix_expression);
        parse_infix_fns.insert(TokenType::LParen, infix_fns::parse_call_expression);
        parse_infix_fns.insert(TokenType::LBracket, infix_fns::parse_index_expression);
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
            TokenType::Let => self.parse_let_var(true),
            TokenType::Var => self.parse_let_var(false),
            TokenType::Ret => self.parse_return(),
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

    pub fn parse_expression(&mut self, precedence: Precedence) -> ParseResult {
        let current_token_type = self.current_token.token_type;
        let mut left_expr = match self.parse_prefix_fns.get(&current_token_type) {
            Some(function) => function(self)?,
            None => {
                let msg = format!("syntax error got {}", self.current_token);
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

    fn parse_let_var(&mut self, is_let: bool) -> ParseResult {
        self.expected_peek(TokenType::Identifier)?;
        let identifier = prefix_fns::parse_identifier_expr(self)?;
        self.expected_peek(TokenType::Assign)?;
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token()
        }
        if is_let {
            Ok(Box::new(LetStatement::new(identifier, expr)))
        } else {
            Ok(Box::new(VarStatement::new(identifier, expr)))
        }
    }

    fn parse_return(&mut self) -> ParseResult {
        self.next_token();
        if self.current_token_is(TokenType::Semicolon) {
            Ok(Box::new(RetStatement::new(None)))
        } else {
            let ret_expr = self.parse_expression(Precedence::Lowest)?;
            self.expected_peek(TokenType::Semicolon)?;
            Ok(Box::new(RetStatement::new(Some(ret_expr))))
        }
    }

    pub fn current_token_is(&mut self, token_type: TokenType) -> bool {
        token_type == self.current_token.token_type
    }

    fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        token_type == self.peek_token.token_type
    }

    pub fn expected_peek(&mut self, token_type: TokenType) -> Result<(), ParseError> {
        if self.peek_token_is(token_type) {
            self.next_token();
            Ok(())
        } else {
            let msg = format!("expected '{}', got {}", token_type, self.peek_token);
            Err(ParseError::Message(msg))
        }
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }
}
