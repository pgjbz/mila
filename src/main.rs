use std::{cell::RefCell, env, fs, process, rc::Rc};

use mila::{
    ast::node::NodeRef,
    evaluator::{environment::Environment, Evaluator},
    lexer::Lexer,
    parser::Parser,
};

fn main() {
    let filename = if let Some(filename) = env::args().nth(1) {
        filename
    } else {
        eprintln!("please use mila filename.mila");
        process::exit(1);
    };
    let source = match fs::read_to_string(&filename) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error on openfile: {:?}", e.kind());
            process::exit(1);
        }
    };
    let lexer = Lexer::new(source, Rc::new(filename));
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if program.errors.is_empty() {
        let eval = Evaluator::new();
        let program: NodeRef = Box::new(program);
        eval.eval(
            Some(&program),
            Rc::new(RefCell::new(Environment::default())),
        );
    } else {
        for error in program.errors {
            eprintln!("{}", error)
        }
    }
}
