use std::rc::Rc;

use mila::{
    ast::node::{
        expressions::{
            bool_expr::BoolExpr, float_expr::FloatExpr, identifier_expr::IdentifierExpr,
            infix_expr::InfixExpr, int_expr::IntExpr, prefix_expr::PrefixExpr,
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

#[test]
fn test_parse_identifier_expr() {
    let mut parser = make_parser("mila".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let identifier = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!("mila", identifier.value);
}

#[test]
fn test_parse_plus_expr() {
    let mut parser = make_parser("1 + 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("+", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_minus_expr() {
    let mut parser = make_parser("1 - 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("-", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_plus_assign_expr() {
    let mut parser = make_parser("a += 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix
        .left
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!("a", left.value, "wrong left value");
    assert_eq!("+=", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_minus_assign_expr() {
    let mut parser = make_parser("1 -= 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("-=", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_asterisk_expr() {
    let mut parser = make_parser("1 * 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("*", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_asterisk_assign_expr() {
    let mut parser = make_parser("1 *= 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("*=", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_slash_expr() {
    let mut parser = make_parser("1 / 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("/", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_slash_assign_expr() {
    let mut parser = make_parser("1 /= 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("/=", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_and_expr() {
    let mut parser = make_parser("true && true".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<BoolExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<BoolExpr>().unwrap();
    assert_eq!(true, right.value, "wrong right value");
    assert_eq!(true, left.value, "wrong left value");
    assert_eq!("&&", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_or_expr() {
    let mut parser = make_parser("true || true".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<BoolExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<BoolExpr>().unwrap();
    assert_eq!(true, right.value, "wrong right value");
    assert_eq!(true, left.value, "wrong left value");
    assert_eq!("||", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_pipe_expr() {
    let mut parser = make_parser("1 | 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("|", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_bitwiseand_expr() {
    let mut parser = make_parser("1 & 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("&", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_shiftleft_expr() {
    let mut parser = make_parser("1 << 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("<<", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_shiftrisght_expr() {
    let mut parser = make_parser("1 >> 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!(">>", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_xor_expr() {
    let mut parser = make_parser("1 ^ 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("^", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_mod_expr() {
    let mut parser = make_parser("1 % 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("%", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_eq_expr() {
    let mut parser = make_parser("1 == 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("==", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_not_eq_expr() {
    let mut parser = make_parser("1 != 1".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let infix = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<InfixExpr>()
        .unwrap();
    let left = infix.left.as_any().downcast_ref::<IntExpr>().unwrap();
    let right = infix.right.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!(1, right.value, "wrong right value");
    assert_eq!(1, left.value, "wrong left value");
    assert_eq!("!=", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_group_expr() {
    let mut parser = make_parser("(1)".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let int = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<IntExpr>()
        .unwrap();
    assert_eq!(1, int.value, "invalid int value");
}

fn make_parser(source: String) -> Parser {
    let lexer = Lexer::new(source, Rc::new("foo.bzr".to_string()));
    Parser::new(lexer)
}
