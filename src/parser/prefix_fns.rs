use std::rc::Rc;

use crate::{
    ast::node::{
        expressions::{
            array_expr::ArrayExpr, bool_expr::BoolExpr, float_expr::FloatExpr, fn_expr::FnExpr,
            identifier_expr::IdentifierExpr, if_expr::IfExpr, int_expr::IntExpr,
            prefix_expr::PrefixExpr, string_expr::StringExpr, while_expr::WhileExpr,
        },
        statements::block_stmt::BlockStatement,
        NodeRef,
    },
    lexer::token::token_type::TokenType,
    parser::precedence::Precedence,
};

use super::{error::ParseError, infix_fns, ParseResult, Parser};

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
    match parser.current_token.token_type {
        TokenType::Number => {
            let value = parser.current_token.value.parse()?;
            Ok(Box::new(IntExpr::new(value)))
        }
        _ => Err(ParseError::Message(format!(
            "expected number got {}",
            parser.current_token
        ))),
    }
}

pub(super) fn parse_float_expr(parser: &mut Parser) -> ParseResult {
    match parser.current_token.token_type {
        TokenType::FloatingPointNumber => {
            let value = parser.current_token.value.parse()?;
            Ok(Box::new(FloatExpr::new(value)))
        }
        _ => Err(ParseError::Message(format!(
            "expected float got {}",
            parser.current_token
        ))),
    }
}

pub(super) fn parse_string_expr(parser: &mut Parser) -> ParseResult {
    match parser.current_token.token_type {
        TokenType::String => {
            let value = parser.current_token.value.clone();
            Ok(Box::new(StringExpr::new(value)))
        }
        _ => Err(ParseError::Message(format!(
            "expected string got {}",
            parser.current_token
        ))),
    }
}

pub(super) fn parse_identifier_expr(parser: &mut Parser) -> ParseResult {
    match parser.current_token.token_type {
        TokenType::Identifier => {
            let value = parser.current_token.value.clone();
            Ok(Box::new(IdentifierExpr::new(value)))
        }
        _ => Err(ParseError::Message(format!(
            "expected identifier got {}",
            parser.current_token
        ))),
    }
}

pub(super) fn parse_group_expr(parser: &mut Parser) -> ParseResult {
    parser.next_token();
    let expr = parser.parse_expression(Precedence::Lowest);
    parser.expected_peek(TokenType::RParen)?;
    expr
}

pub(super) fn parse_block_stmt(parser: &mut Parser) -> ParseResult {
    parser.next_token();
    let mut stmts = Vec::new();
    while !parser.current_token_is(TokenType::Eof) && !parser.current_token_is(TokenType::RBrace) {
        stmts.push(parser.parse_statement()?);
        parser.next_token();
    }
    let block_stmt = BlockStatement::new(Rc::new(stmts));
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

pub(super) fn parse_fn_expr(parser: &mut Parser) -> ParseResult {
    let name = match parser.expected_peek(TokenType::Identifier) {
        Ok(_) => {
            let name = Some(Rc::new(parse_identifier_expr(parser)?));
            name
        }
        Err(_) => None,
    };
    parser.expected_peek(TokenType::LParen)?;
    let parameters = Rc::new(parse_function_parameters(parser)?);
    let body = Rc::new(parse_block_stmt(parser)?);
    Ok(Box::new(FnExpr::new(body, name, parameters)))
}

fn parse_function_parameters(parser: &mut Parser) -> Result<Vec<NodeRef>, ParseError> {
    let mut parameters = Vec::with_capacity(4);
    match parser.expected_peek(TokenType::RParen) {
        Ok(_) => {
            parser.next_token();
            Ok(parameters)
        }
        Err(_) => {
            parser.next_token();
            let parameter = parse_identifier_expr(parser)?;
            parameters.push(parameter);
            while parser.peek_token_is(TokenType::Comma) {
                parser.next_token();
                parser.next_token();
                let parameter = parse_identifier_expr(parser)?;
                parameters.push(parameter);
            }
            parser.expected_peek(TokenType::RParen)?;
            Ok(parameters)
        }
    }
}

pub(super) fn parse_array_expr(parser: &mut Parser) -> ParseResult {
    let elements = infix_fns::parse_expr_list(parser, TokenType::RBracket)?;
    Ok(Box::new(ArrayExpr::new(elements)))
}
