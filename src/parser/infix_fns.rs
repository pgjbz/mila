use crate::ast::node::{expressions::infix_expr::InfixExpr, Node};

use super::{ParseResult, Parser};

use crate::precedence;

pub(super) fn parse_infix_expression(parser: &mut Parser, left: Box<dyn Node>) -> ParseResult {
    parser.next_token();
    let precedence = precedence!(parser.current_token.token_type);
    let operator = parser.current_token.value.clone();
    parser.next_token();
    let right = parser.parse_expression(precedence)?;
    Ok(Box::new(InfixExpr::new(operator, right, left)))
}

pub(super) fn parse_call_expression(parser: &mut Parser, left: Box<dyn Node>) -> ParseResult {
    todo!()
}