use std::rc::Rc;

use mila::{
    ast::node::{
        expressions::{
            array_expr::ArrayExpr, bool_expr::BoolExpr, call_expr::CallExpr, float_expr::FloatExpr,
            fn_expr::FnExpr, identifier_expr::IdentifierExpr, if_expr::IfExpr,
            index_expr::IndexExpr, infix_expr::InfixExpr, int_expr::IntExpr,
            prefix_expr::PrefixExpr, string_expr::StringExpr, while_expr::WhileExpr,
        },
        statements::{
            block_stmt::BlockStatement, expression_stmt::ExpressionStmt, let_stmt::LetStatement,
            ret_stmt::RetStatement, var_stmt::VarStatement,
        },
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

#[test]
fn test_parse_let_stmt() {
    let mut parser = make_parser("let a = 10;".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let let_stmt = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<LetStatement>()
        .unwrap();
    let int = let_stmt.value.as_any().downcast_ref::<IntExpr>().unwrap();
    let identifier = let_stmt
        .name
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!(10, int.value, "invalid int value");
    assert_eq!("a", identifier.value, "invalid name value");
}

#[test]
fn test_parse_var_stmt() {
    let mut parser = make_parser("var a = 10;".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let let_stmt = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<VarStatement>()
        .unwrap();
    let int = let_stmt.value.as_any().downcast_ref::<IntExpr>().unwrap();
    let identifier = let_stmt
        .name
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!(10, int.value, "invalid int value");
    assert_eq!("a", identifier.value, "invalid name value");
}

#[test]
fn test_parse_ret_with_value_stmt() {
    let mut parser = make_parser("ret 10;".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let ret_stmt = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<RetStatement>()
        .unwrap();
    let int = ret_stmt
        .value
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<IntExpr>()
        .unwrap();
    assert_eq!(10, int.value, "invalid int value");
}

#[test]
fn test_parse_ret_without_value_stmt() {
    let mut parser = make_parser("ret;".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let ret_stmt = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<RetStatement>()
        .unwrap();
    assert!(ret_stmt.value.is_none());
}

#[test]
fn test_parse_block_expr() {
    let mut parser = make_parser("{ let a = 10; let b = 20; }".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let block_stmt = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();

    assert_eq!(2, block_stmt.statements.len());
}

#[test]
fn test_parse_if_expr() {
    let mut parser = make_parser("if true { 10 } else { 10 }".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let if_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<IfExpr>()
        .unwrap();
    let consequence = if_expr
        .consequence
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();
    let alternative = if_expr
        .alternative
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();
    assert_eq!(
        1,
        consequence.statements.len(),
        "wrong number of statements in consequence block"
    );
    assert_eq!(
        1,
        alternative.statements.len(),
        "wrong number of statements in alternative block"
    );
}

#[test]
fn test_parse_if_else_if_expr() {
    let mut parser = make_parser("if true { 10 } else if true { 10 }".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let if_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<IfExpr>()
        .unwrap();
    let consequence = if_expr
        .consequence
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();
    let el_if = if_expr
        .el_if
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<IfExpr>()
        .unwrap();
    let el_if_consequence = el_if
        .consequence
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();
    assert_eq!(
        1,
        consequence.statements.len(),
        "wrong number of statements in consequence block"
    );
    assert_eq!(
        1,
        el_if_consequence.statements.len(),
        "wrong number of statements in alternative block"
    );
}

#[test]
fn test_dot_expr() {
    let mut parser = make_parser("obj.att".to_string());
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
    let right = infix
        .right
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!("att", right.value, "wrong right value");
    assert_eq!("obj", left.value, "wrong left value");
    assert_eq!(".", infix.operator, "wrong operator value");
}

#[test]
fn test_parse_while_expr() {
    let mut parser = make_parser("while true { 10 } ".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let while_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<WhileExpr>()
        .unwrap();
    let consequence = while_expr
        .consequence
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();
    assert_eq!(
        1,
        consequence.statements.len(),
        "wrong number of statements in consequence block"
    );
}

#[test]
fn test_parse_fn_with_two_parameters_expr() {
    let mut parser =
        make_parser("fn mila(param, param) { 10 } fn b (a, b, c) {} mila(1, 2)".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(3, statemets.len(), "wrong number of statemets");
    let fn_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<FnExpr>()
        .unwrap();
    let body = fn_expr
        .body
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();
    let paratemers_len = fn_expr.parameters.len();
    let name = fn_expr
        .name
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!(
        1,
        body.statements.len(),
        "wrong number of statements in body function"
    );
    assert_eq!(2, paratemers_len, "wrong number of parameters in function");
    assert_eq!("mila", name.value, "wrong name of function");
}

#[test]
fn test_parse_fn_with_one_parameters_expr() {
    let mut parser = make_parser("fn mila(param) { 10 }".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let fn_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<FnExpr>()
        .unwrap();
    let body = fn_expr
        .body
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();
    let paratemers_len = fn_expr.parameters.len();
    let name = fn_expr
        .name
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!(
        1,
        body.statements.len(),
        "wrong number of statements in body function"
    );
    assert_eq!(1, paratemers_len, "wrong number of parameters in function");
    assert_eq!("mila", name.value, "wrong name of function");
}

#[test]
fn test_parse_fn_with_zero_parameters_expr() {
    let mut parser = make_parser("fn mila() { 10 }".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let fn_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<FnExpr>()
        .unwrap();
    let body = fn_expr
        .body
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();
    let paratemers_len = fn_expr.parameters.len();
    let name = fn_expr
        .name
        .as_ref()
        .unwrap()
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!(
        1,
        body.statements.len(),
        "wrong number of statements in body function"
    );
    assert_eq!(0, paratemers_len, "wrong number of parameters in function");
    assert_eq!("mila", name.value, "wrong name of function");
}

#[test]
fn test_parse_fn_without_name_expr() {
    let mut parser = make_parser("fn () { 10 }".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let fn_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<FnExpr>()
        .unwrap();
    let body = fn_expr
        .body
        .as_any()
        .downcast_ref::<BlockStatement>()
        .unwrap();
    let paratemers_len = fn_expr.parameters.len();
    let name = fn_expr.name.is_none();
    assert_eq!(
        1,
        body.statements.len(),
        "wrong number of statements in body function"
    );
    assert_eq!(0, paratemers_len, "wrong number of parameters in function");
    assert!(name, "wrong name of function");
}

#[test]
fn test_parse_let_with_function_stmt() {
    let mut parser = make_parser("let a = fn (a, b) { 1 + 2 };".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let let_stmt = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<LetStatement>()
        .unwrap();
    let is_function = let_stmt.value.as_any().downcast_ref::<FnExpr>().is_some();
    let identifier = let_stmt
        .name
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert!(is_function, "is not a function");
    assert_eq!("a", identifier.value, "invalid name value");
}
#[test]
fn test_parse_fn_with_zero_parameters_expr_should_be_error() {
    let mut parser = make_parser("fn 1() { 10 }".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(1, errors.len(), "wrong number of errors");
    assert_eq!(2, statemets.len(), "wrong number of statemets");
}

#[test]
fn test_parse_let_stmt_should_fail_without_identifier() {
    let mut parser = make_parser("let = 10;".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(2, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
}

#[test]
fn test_parse_var_stmt_should_fail_without_identifier() {
    let mut parser = make_parser("var = 10;".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(2, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
}

#[test]
fn test_parse_let_stmt_should_fail_without_assign() {
    let mut parser = make_parser("let mila 10;".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(1, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
}

#[test]
fn test_parse_var_stmt_should_fail_without_assign() {
    let mut parser = make_parser("var mila 10;".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(1, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
}

#[test]
fn test_call_expr_with_zero_args() {
    let mut parser = make_parser("fn mila() { 1 } mila();".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(2, statemets.len(), "wrong number of statemets");
    let call_expr = statemets
        .get(1)
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<CallExpr>()
        .unwrap();
    let args_len = call_expr.arguments.len();
    let function = call_expr
        .function
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!(0, args_len, "wrong number of arguments in function call");
    assert_eq!("mila", function.value, "wrong name of function");
}

#[test]
fn test_call_expr_with_one_args() {
    let mut parser = make_parser("fn mila(a) { a } mila(1);".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(2, statemets.len(), "wrong number of statemets");
    let call_expr = statemets
        .get(1)
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<CallExpr>()
        .unwrap();
    let args_len = call_expr.arguments.len();
    let function = call_expr
        .function
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!(1, args_len, "wrong number of arguments in function call");
    assert_eq!("mila", function.value, "wrong name of function");
}

#[test]
fn test_call_expr_with_two_args() {
    let mut parser = make_parser("mila(1, 0);".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let call_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<CallExpr>()
        .unwrap();
    let args_len = call_expr.arguments.len();
    let function = call_expr
        .function
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    assert_eq!(2, args_len, "wrong number of arguments in function call");
    assert_eq!("mila", function.value, "wrong name of function");
}

#[test]
fn test_dot_expr_with_call_expr() {
    let mut parser = make_parser("obj.method();".to_string());
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
    let right = infix.right.as_any().downcast_ref::<CallExpr>().is_some();
    assert!(right, "right value has to be a call expr");
    assert_eq!("obj", left.value, "wrong left value");
    assert_eq!(".", infix.operator, "wrong operator value");
}

#[test]
fn test_index_expr() {
    let mut parser = make_parser("a[1]".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let index_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<IndexExpr>()
        .unwrap();
    let left = index_expr
        .left
        .as_any()
        .downcast_ref::<IdentifierExpr>()
        .unwrap();
    let index = index_expr.index.as_any().downcast_ref::<IntExpr>().unwrap();
    assert_eq!("a", left.value, "wrong left value");
    assert_eq!(1, index.value, "wrong index value");
}

#[test]
fn test_parse_array_with_zero_item() {
    let mut parser = make_parser("[]".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let array_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<ArrayExpr>()
        .unwrap();
    let values_len = array_expr.values.len();
    assert_eq!(0, values_len, "wrong number os values value");
}

#[test]
fn test_parse_array_with_one_item() {
    let mut parser = make_parser("[1]".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let array_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<ArrayExpr>()
        .unwrap();
    let values_len = array_expr.values.len();
    assert_eq!(1, values_len, "wrong number os values value");
}

#[test]
fn test_parse_array_with_two_item() {
    let mut parser = make_parser("[1, 2]".to_string());
    let program = parser.parse_program();
    let statemets = program.statements;
    let errors = program.errors;
    assert_eq!(0, errors.len(), "wrong number of errors");
    assert_eq!(1, statemets.len(), "wrong number of statemets");
    let array_expr = statemets
        .first()
        .unwrap()
        .as_any()
        .downcast_ref::<ExpressionStmt>()
        .unwrap()
        .expression
        .as_any()
        .downcast_ref::<ArrayExpr>()
        .unwrap();
    let values_len = array_expr.values.len();
    assert_eq!(2, values_len, "wrong number os values value");
}

fn make_parser(source: String) -> Parser {
    let lexer = Lexer::new(source, Rc::new("foo.bzr".to_string()));
    Parser::new(lexer)
}
