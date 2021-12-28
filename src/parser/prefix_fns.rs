use crate::{
    ast::node::{
        expressions::{
            bool_expr::BoolExpr, float_expr::FloatExpr, identifier_expr::IdentifierExpr,
            if_expr::IfExpr, int_expr::IntExpr, prefix_expr::PrefixExpr, string_expr::StringExpr,
            while_expr::WhileExpr,
        },
        statements::block_stmt::BlockStatement,
    },
    lexer::token::token_type::TokenType,
    parser::precedence::Precedence,
};

use super::{error::ParseError, ParseResult, Parser};

pub(super) fn parse_prefix_expr(parser: &mut Parser) -> ParseResult {
    let operator = parser.current_token.value.clone();
    parser.next_token();
    let right = parser.parse_expression(Precedence::Prefix)?;
    Ok(Box::new(PrefixExpr::new(operator, right)))
}

pub(super) fn parse_boolean_expr(parser: &mut Parser) -> ParseResult {
    let value = match parser.current_token.token_type {
        TokenType::True => true,
        TokenType::False => false,
        _ => {
            let msg = format!("expected boolean value got: {}", parser.current_token);
            return Err(ParseError::Message(msg));
        }
    };
    Ok(Box::new(BoolExpr::new(value)))
}

pub(super) fn parse_int_expr(parser: &mut Parser) -> ParseResult {
    let value = parser.current_token.value.parse()?;
    Ok(Box::new(IntExpr::new(value)))
}

pub(super) fn parse_float_expr(parser: &mut Parser) -> ParseResult {
    let value = parser.current_token.value.parse()?;
    Ok(Box::new(FloatExpr::new(value)))
}

pub(super) fn parse_string_expr(parser: &mut Parser) -> ParseResult {
    let value = parser.current_token.value.clone();
    Ok(Box::new(StringExpr::new(value)))
}

pub(super) fn parse_identifier_expr(parser: &mut Parser) -> ParseResult {
    let value = parser.current_token.value.clone();
    Ok(Box::new(IdentifierExpr::new(value)))
}

pub(super) fn parse_group_expr(parser: &mut Parser) -> ParseResult {
    parser.next_token();
    let expr = parser.parse_expression(Precedence::Lowest);
    parser.expected_peek(TokenType::RParen)?;
    expr
}

pub(super) fn parse_block_stmt(parser: &mut Parser) -> ParseResult {
    parser.next_token();
    let mut block_stmt = BlockStatement::new();
    while !parser.current_token_is(TokenType::Eof) && !parser.current_token_is(TokenType::RBrace) {
        block_stmt.push_stmt(parser.parse_statement()?);
        parser.next_token();
    }
    Ok(Box::new(block_stmt))
}

pub(super) fn parse_if_expr(parser: &mut Parser) -> ParseResult {
    parser.next_token();
    let condition = parser.parse_expression(Precedence::Lowest)?;
    parser.expected_peek(TokenType::LBrace)?;
    let consequence = parse_block_stmt(parser)?;
    let mut if_expr = IfExpr::new(condition, consequence);
    if parser.peek_token_is(TokenType::Else) {
        parser.next_token();
        match parser.expected_peek(TokenType::LBrace) {
            Ok(_) => {
                let alternative = parse_block_stmt(parser)?;
                if_expr.alternative = Some(alternative)
            }
            Err(_) => match parser.expected_peek(TokenType::If) {
                Ok(_) => if_expr.el_if = Some(parse_if_expr(parser)?),
                Err(e) => return Err(e),
            },
        }
    }
    Ok(Box::new(if_expr))
}

pub(super) fn parse_while_expr(parser: &mut Parser) -> ParseResult {
    parser.next_token();
    let condition = parser.parse_expression(Precedence::Lowest)?;
    parser.expected_peek(TokenType::LBrace)?;
    let consequence = parse_block_stmt(parser)?;
    let while_expr = WhileExpr::new(condition, consequence);
    Ok(Box::new(while_expr))
}
