use std::rc::Rc;

use mila::{
    ast::node::NodeRef,
    evaluator::{
        environment::Environment,
        objects::{
            boolean::Boolean, eval_error::EvalError, float::Float, function::Function,
            integer::Integer, string::Str, ObjectRef,
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
    for source in tests {
        let evaluated = test_eval(source);
        let evaluated = evaluated.as_any().downcast_ref::<Function>().is_some();
        assert!(evaluated, "invalid value")
    }
}

fn test_eval(source: String) -> Rc<ObjectRef> {
    let lexer = Lexer::new(source, Rc::new("foo.bzr".to_string()));
    let mut parser = Parser::new(lexer);
    let program: NodeRef = Box::new(parser.parse_program());
    let eval: Evaluator = Default::default();
    let mut env: Environment = Default::default();
    eval.eval(Some(&program), &mut env).unwrap()
}
