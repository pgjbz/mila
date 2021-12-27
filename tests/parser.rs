use std::rc::Rc;

use mila::{
    ast::node::{
        expressions::{
            bool_expr::BoolExpr, float_expr::FloatExpr, int_expr::IntExpr, prefix_expr::PrefixExpr,
            string_expr::StringExpr,
        },
        statements::expression_stmt::ExpressionStmt,
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_parse_bang_expr() {
    let mut parser = make_parser("!true".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let prefix_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<PrefixExpr>()
        .unwrap();
    assert_eq!("!".to_string(), prefix_expr.operator);
    let prefix_bool_value = prefix_expr
        .right
        .as_any()
        .downcast_ref::<BoolExpr>()
        .unwrap();
    assert_eq!(true, prefix_bool_value.value);
}

#[test]
fn test_parse_bool_expr() {
    let mut parser = make_parser("true".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let bool_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<BoolExpr>()
        .unwrap();
    assert_eq!(true, bool_expr.value);
}

#[test]
fn test_parse_int_expr() {
    let mut parser = make_parser("10".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let int_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<IntExpr>()
        .unwrap();
    assert_eq!(10, int_expr.value);
}

#[test]
fn test_parse_float_expr() {
    let mut parser = make_parser("10.0".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let float_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<FloatExpr>()
        .unwrap();
    assert_eq!(10.0, float_expr.value);
}

#[test]
fn test_parse_string_expr() {
    let mut parser = make_parser("\"10.0\"".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let float_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<StringExpr>()
        .unwrap();
    assert_eq!("10.0", float_expr.value);
}

fn make_parser(source: String) -> Parser {
    let lexer = Lexer::new(source, Rc::new("foo.bzr".to_string()));
    Parser::new(lexer)
}
