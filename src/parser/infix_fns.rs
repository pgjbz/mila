use crate::{
    ast::node::{
        expressions::{call_expr::CallExpr, index_expr::IndexExpr, infix_expr::InfixExpr},
        Node,
    },
    lexer::token::token_type::TokenType,
    parser::precedence::Precedence,
};

use super::{error::ParseError, ParseResult, Parser};

use crate::precedence;

pub(super) fn parse_infix_expression(parser: &mut Parser, left: Box<dyn Node>) -> ParseResult {
    parser.next_token();
    let precedence = precedence!(parser.current_token.token_type);
    let operator = parser.current_token.value.clone();
    parser.next_token();
    let right = parser.parse_expression(precedence)?;
    Ok(Box::new(InfixExpr::new(operator, right, left)))
}

pub(super) fn parse_call_expression(parser: &mut Parser, function: Box<dyn Node>) -> ParseResult {
    parser.next_token();
    let args = parse_expr_list(parser, TokenType::RParen)?;
    Ok(Box::new(CallExpr::new(function, args)))
}

pub(super) fn parse_index_expression(parser: &mut Parser, left: Box<dyn Node>) -> ParseResult {
    parser.next_token();
    parser.next_token();
    let index = parser.parse_expression(Precedence::Lowest)?;
    parser.expected_peek(TokenType::RBracket)?;
    Ok(Box::new(IndexExpr::new(left, index)))
}

pub fn parse_expr_list(
    parser: &mut Parser,
    end: TokenType,
) -> Result<Vec<Box<dyn Node>>, ParseError> {
    let mut exprs = Vec::with_capacity(3);
    if parser.peek_token_is(end) {
        parser.next_token();
        return Ok(exprs);
    }
    parser.next_token();
    exprs.push(parser.parse_expression(Precedence::Lowest)?);
    while parser.peek_token_is(TokenType::Comma) {
        parser.next_token();
        parser.next_token();
        exprs.push(parser.parse_expression(Precedence::Lowest)?);
    }
    parser.expected_peek(end)?;
    Ok(exprs)
}
