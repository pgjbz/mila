use std::rc::Rc;

use mila::{
    ast::node::NodeRef,
    evaluator::{
        environment::Environment,
        objects::{integer::Integer, ObjectRef},
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

fn test_eval(source: String) -> ObjectRef {
    let lexer = Lexer::new(source, Rc::new("foo.bzr".to_string()));
    let mut parser = Parser::new(lexer);
    let program: NodeRef = Box::new(parser.parse_program());
    let eval: Evaluator = Default::default();
    let mut env: Environment = Default::default();
    eval.eval(Some(&program), &mut env).unwrap()
}
