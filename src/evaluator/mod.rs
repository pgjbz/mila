use std::process;

use crate::ast::{
    node::{
        expressions::int_expr::IntExpr, statements::expression_stmt::ExpressionStmt, NodeRef,
        OpCode,
    },
    Program,
};

use self::{
    environment::Environment,
    objects::{integer::Integer, ObjectRef},
};

pub mod environment;
pub mod objects;

#[derive(Default)]
pub struct Evaluator;

impl Evaluator {
    pub fn eval(&self, node: Option<&NodeRef>, enviroment: &mut Environment) -> Option<ObjectRef> {
        if let Some(node) = node {
            match node.get_op_code() {
                OpCode::If => todo!(),
                OpCode::Let => todo!(),
                OpCode::Var => todo!(),
                OpCode::Ret => todo!(),
                OpCode::Int => {
                    let int_expr = node.as_any().downcast_ref::<IntExpr>().unwrap();
                    Some(Box::new(Integer::new(int_expr.value)))
                }
                OpCode::Call => todo!(),
                OpCode::Bool => todo!(),
                OpCode::Array => todo!(),
                OpCode::Index => todo!(),
                OpCode::Block => todo!(),
                OpCode::Infix => todo!(),
                OpCode::Float => todo!(),
                OpCode::While => todo!(),
                OpCode::Prefix => todo!(),
                OpCode::String => todo!(),
                OpCode::Program => {
                    let program = node.as_any().downcast_ref::<Program>().unwrap();
                    Some(self.eval_statements(&program.statements, enviroment))
                }
                OpCode::Function => todo!(),
                OpCode::Expression => {
                    let expr = node.as_any().downcast_ref::<ExpressionStmt>().unwrap();
                    self.eval(Some(&expr.expression), enviroment)
                }
                OpCode::Identifier => todo!(),
            }
        } else {
            None
        }
    }

    fn eval_statements(&self, stmts: &[NodeRef], enviroment: &mut Environment) -> ObjectRef {
        let mut result = None;
        for stmt in stmts.iter() {
            result = self.eval(Some(stmt), enviroment);
        }
        if let Some(result) = result {
            result
        } else {
            process::exit(1)
        }
    }
}
