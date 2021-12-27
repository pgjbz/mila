use crate::{
    ast::node::{
        expressions::{
            bool_expr::BoolExpr, float_expr::FloatExpr, identifier_expr::IdentifierExpr,
            int_expr::IntExpr, prefix_expr::PrefixExpr, string_expr::StringExpr,
        },
        Node,
    },
    lexer::token::token_type::TokenType,
    parser::precedence::Precedence,
};

use super::{error::ParseError, Parser};

pub(super) fn parse_prefix_expr(parser: &mut Parser) -> Result<Box<dyn Node>, ParseError> {
    let operator = parser.current_token.value.clone();
    parser.next_token();
    let right = parser.parse_expression(Precedence::Prefix)?;
    Ok(Box::new(PrefixExpr::new(operator, right)))
}

pub(super) fn parse_boolean_expr(parser: &mut Parser) -> Result<Box<dyn Node>, ParseError> {
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

pub(super) fn parse_int_expr(parser: &mut Parser) -> Result<Box<dyn Node>, ParseError> {
    let value = parser.current_token.value.parse()?;
    Ok(Box::new(IntExpr::new(value)))
}

pub(super) fn parse_float_expr(parser: &mut Parser) -> Result<Box<dyn Node>, ParseError> {
    let value = parser.current_token.value.parse()?;
    Ok(Box::new(FloatExpr::new(value)))
}

pub(super) fn parse_string_expr(parser: &mut Parser) -> Result<Box<dyn Node>, ParseError> {
    let value = parser.current_token.value.clone();
    Ok(Box::new(StringExpr::new(value)))
}

pub(super) fn parse_identifier_expr(parser: &mut Parser) -> Result<Box<dyn Node>, ParseError> {
    let value = parser.current_token.value.clone();
    Ok(Box::new(IdentifierExpr::new(value)))
}
