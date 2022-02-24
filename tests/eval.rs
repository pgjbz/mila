use std::{cell::RefCell, rc::Rc};

use mila::{
    ast::node::NodeRef,
    evaluator::{
        objects::{
            array::Array, boolean::Boolean, eval_error::EvalError, float::Float,
            function::Function, integer::Integer, string::Str, ObjectRef,
        },
        Evaluator,
    },
    lexer::Lexer,
    parser::Parser,
};

#[test]
fn test_eval_int_expr() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("10".to_string(), 10));
    tests.push(("5".to_string(), 5));
    tests.push(("1".to_string(), 1));
    tests.push(("2".to_string(), 2));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_block_expr() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("{ 10 }".to_string(), 10));
    tests.push(("{ 5 }".to_string(), 5));
    tests.push(("{ 1 }".to_string(), 1));
    tests.push(("{ 2 }".to_string(), 2));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_prefix_expr() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("-10".to_string(), -10));
    tests.push(("--5".to_string(), 5));
    tests.push(("!1".to_string(), -2));
    tests.push(("!2".to_string(), -3));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_infix_expr() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("10 + 10".to_string(), 20));
    tests.push(("5 + 5 - 2".to_string(), 8));
    tests.push(("10 - 5 * 3".to_string(), -5));
    tests.push(("(10 - 5) * 3".to_string(), 15));
    tests.push(("10 / 10".to_string(), 1));
    tests.push(("2 << 1".to_string(), 4));
    tests.push(("4 >> 1".to_string(), 2));
    tests.push(("7 & 1".to_string(), 1));
    tests.push(("7 | 1".to_string(), 7));
    tests.push(("7 ^ 1".to_string(), 6));
    tests.push(("10 % 2".to_string(), 0));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_infix_expr_error() {
    let mut tests: Vec<(String, String)> = Vec::new();
    tests.push((
        "10.0 + 10".to_string(),
        "unsoported operation float + int".to_string(),
    ));
    tests.push((
        "10.0 - 10".to_string(),
        "unsoported operation float - int".to_string(),
    ));
    tests.push((
        "10 + 10.0".to_string(),
        "unsoported operation int + float".to_string(),
    ));
    tests.push((
        "10 - 10.0".to_string(),
        "unsoported operation int - float".to_string(),
    ));
    tests.push((
        "10.0 + false".to_string(),
        "unsoported operation float + bool".to_string(),
    ));
    tests.push((
        "10.0 - false".to_string(),
        "unsoported operation float - bool".to_string(),
    ));
    tests.push((
        "10 + false".to_string(),
        "unsoported operation int + bool".to_string(),
    ));
    tests.push((
        "10 - false".to_string(),
        "unsoported operation int - bool".to_string(),
    ));
    tests.push((
        "10 || 10".to_string(),
        "unsoported operation int || int".to_string(),
    ));
    tests.push((
        "10 && 10".to_string(),
        "unsoported operation int && int".to_string(),
    ));
    tests.push((
        "10.0 || 10.0".to_string(),
        "unsoported operation float || float".to_string(),
    ));
    tests.push((
        "10.0 && 10.0".to_string(),
        "unsoported operation float && float".to_string(),
    ));
    tests.push((
        "10.0 & 10.0".to_string(),
        "unsoported operation float & float".to_string(),
    ));
    tests.push((
        "10.0 | 10.0".to_string(),
        "unsoported operation float | float".to_string(),
    ));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<EvalError>().unwrap();
        assert_eq!(expected, evaluated.message, "invalid value")
    }
}

#[test]
fn test_eval_infix_with_float_expr() {
    let mut tests: Vec<(String, f64)> = Vec::new();
    tests.push(("10.0 + 10.0".to_string(), 20.0));
    tests.push(("5.0 - 1.0".to_string(), 4.0));
    tests.push(("5.0 - 6.0".to_string(), -1.0));
    tests.push(("5.0 / 5.0".to_string(), 1.0));
    tests.push(("5.0 * 5.0".to_string(), 25.0));
    tests.push(("5.0 * 5.0 + 10.0 / 10.0".to_string(), 26.0));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Float>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_infix_comparator_expr() {
    let mut tests: Vec<(String, bool)> = Vec::new();
    tests.push(("10 == 10".to_string(), true));
    tests.push(("10 >= 10".to_string(), true));
    tests.push(("10 <= 10".to_string(), true));
    tests.push(("10 > 10".to_string(), false));
    tests.push(("10 < 10".to_string(), false));
    tests.push(("8 < 10".to_string(), true));
    tests.push(("8 > 2".to_string(), true));
    tests.push(("8.0 > 2.0".to_string(), true));
    tests.push(("8.0 < 2.0".to_string(), false));
    tests.push(("10.0 == 10.0".to_string(), true));
    tests.push(("10.0 >= 10.0".to_string(), true));
    tests.push(("10.0 <= 10.0".to_string(), true));

    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Boolean>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_float_expr() {
    let mut tests: Vec<(String, f64)> = Vec::new();
    tests.push(("10.0".to_string(), 10.0));
    tests.push(("5.0".to_string(), 5.0));
    tests.push(("1.0".to_string(), 1.0));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Float>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_float_prefix_expr() {
    let mut tests: Vec<(String, f64)> = Vec::new();
    tests.push(("-10.0".to_string(), -10.0));
    tests.push(("--5.0".to_string(), 5.0));
    tests.push(("1.0".to_string(), 1.0));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Float>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_boolean_expr() {
    let mut tests: Vec<(String, bool)> = Vec::new();
    tests.push(("true".to_string(), true));
    tests.push(("false".to_string(), false));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Boolean>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_prefix_boolean_expr() {
    let mut tests: Vec<(String, bool)> = Vec::new();
    tests.push(("!true".to_string(), false));
    tests.push(("!false".to_string(), true));
    tests.push(("!!false".to_string(), false));
    tests.push(("!!true".to_string(), true));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Boolean>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_infix_boolean_expr() {
    let mut tests: Vec<(String, bool)> = Vec::new();
    tests.push(("true && true".to_string(), true));
    tests.push(("true || true".to_string(), true));
    tests.push(("true && false".to_string(), false));
    tests.push(("true || false".to_string(), true));
    tests.push(("false || false".to_string(), false));
    tests.push(("!false || false".to_string(), true));
    tests.push(("true && !false".to_string(), true));
    tests.push(("true && !false || false".to_string(), true));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Boolean>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_str_expr() {
    let mut tests: Vec<(String, String)> = Vec::new();
    tests.push(("\"mila\"".to_string(), "mila".to_string()));
    tests.push(("\"lang\"".to_string(), "lang".to_string()));
    tests.push(("\"false\"".to_string(), "false".to_string()));
    tests.push(("\"true\"".to_string(), "true".to_string()));
    tests.push(("\"10\"".to_string(), "10".to_string()));
    tests.push(("\"10.0\"".to_string(), "10.0".to_string()));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Str>().unwrap();
        assert_eq!(expected, evaluated.value, "invalid value")
    }
}

#[test]
fn test_let_stmt() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("let a = 10; a;".to_string(), 10));
    tests.push(("let a = 5; let b = 10; a;".to_string(), 5));
    tests.push(("let a = 1; let b = 4; let c = 10; a;".to_string(), 1));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_var_stmt() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("var a = 10; a;".to_string(), 10));
    tests.push(("var a = 5; let b = 10; a;".to_string(), 5));
    tests.push(("var a = 1; let b = 4; let c = 10; a;".to_string(), 1));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_get_unnexistent_identifier_should_be_error() {
    let mut tests: Vec<(String, String)> = Vec::new();
    tests.push(("a;".to_string(), "unknown word 'a'".to_string()));
    tests.push(("b;".to_string(), "unknown word 'b'".to_string()));
    tests.push(("mila;".to_string(), "unknown word 'mila'".to_string()));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<EvalError>().unwrap();
        assert_eq!(expected, evaluated.message, "invalid value")
    }
}

#[test]
fn test_eval_function() {
    let mut tests: Vec<String> = Vec::new();
    tests.push("fn mila() {} mila;".to_string());
    tests.push("fn mila(a) {}".to_string());
    tests.push("fn mila(a, b) {} fn mila2(){} mila;".to_string());
    tests.push("let sum = fn (a, b) { a + b } sum;".to_string());
    for source in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Function>().is_some();
        assert!(evaluated, "invalid value")
    }
}

#[test]
fn test_eval_array() {
    let mut tests: Vec<(String, usize)> = Vec::new();
    tests.push(("[1]".to_string(), 1));
    tests.push(("[1, true]".to_string(), 2));
    tests.push(("[1,1,3]".to_string(), 3));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Array>().unwrap();
        let value = evaluated.values.borrow().len();
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_if_expr() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("if true { 1 } else { 2 }".to_string(), 1));
    tests.push(("if false { 2 } else { 1 }".to_string(), 1));
    tests.push((
        "if false { 2 } else if true { 1 } else { 3 }".to_string(),
        1,
    ));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_call_expr() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("fn simple() { ret 1; }; simple()".to_string(), 1));
    tests.push(("fn simple(a) { a } simple(1);".to_string(), 1));
    tests.push(("fn sum(a, b) { a + b; } sum(1, 2);".to_string(), 3));
    tests.push(("fn sum(a, b) { ret 1; a + b; } sum(1, 2);".to_string(), 1));
    tests.push((
        "fn test() { if true { 1 } else { 2 } } test();".to_string(),
        1,
    ));
    tests.push(("let sum = fn (a, b) { a + b }} sum(1, 2);".to_string(), 3));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_built_in_expr() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("len(\"abc\")".to_string(), 3));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_built_str_in_expr() {
    let mut tests: Vec<(String, String)> = Vec::new();
    tests.push(("puts(\"abc\")".to_string(), "abc".to_string()));
    tests.push(("eputs(\"abc\")".to_string(), "abc".to_string()));
    tests.push(("putsln(\"abc\")".to_string(), "abc".to_string()));
    tests.push(("eputsln(\"abc\")".to_string(), "abc".to_string()));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Str>().unwrap();
        let value = evaluated.value.clone();
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_while_expr() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push((
        "let a = 1; while a < 10 { let a = a + 1; } a;".to_string(),
        10,
    ));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_index_arr() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push(("let arr = [1, 2, 3, 10, 4]; arr[3]".to_string(), 10));
    tests.push(("[1, 2, 3, 10, 4][3]".to_string(), 10));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_index_string() {
    let mut tests: Vec<(String, String)> = Vec::new();
    tests.push(("let arr = \"abcd\"; arr[3]".to_string(), "d".to_string()));
    tests.push(("\"abcd\"[3]".to_string(), "d".to_string()));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Str>().unwrap();
        let value = evaluated.value.clone();
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]
fn test_eval_arr_function() {
    let mut tests: Vec<(String, isize)> = Vec::new();
    tests.push((
        "let arr = [1, 2, 3, 10, 4]; arr.push(10); arr[5]".to_string(),
        10,
    ));
    tests.push((
        "let arr = [1, 2, 3, 10, 4]; arr.replace(0, 10); arr[0]".to_string(),
        10,
    ));
    for (source, expected) in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Integer>().unwrap();
        let value = evaluated.value;
        assert_eq!(expected, value, "invalid value")
    }
}

#[test]

fn test_eval_hash_obj() {
    let tests = vec![
        ("| fruit: \"banana\", sum: 1 + 2, |[\"fruit\"]", "banana"),
        ("let obj = | array: [1, 2, 3], |; obj[\"array\"]", "[1,2,3]"),
    ];
    for (source, expected) in tests {
        let evalueted = test_eval(source.to_string());
        assert_eq!(expected.to_string(), evalueted.to_string())
    }
}

fn test_eval(source: String) -> ObjectRef {
    let lexer = Lexer::new(source, Rc::new("foo.bzr".to_string()));
    let mut parser = Parser::new(lexer);
    let program: NodeRef = Box::new(parser.parse_program());
    let eval: Evaluator = Evaluator::new();
    let env = Rc::new(RefCell::new(Default::default()));
    eval.eval(Some(&program), env).unwrap()
}
