use std::{process, rc::Rc};

use crate::ast::{
    node::{
        expressions::{
            bool_expr::BoolExpr, float_expr::FloatExpr, identifier_expr::IdentifierExpr,
            int_expr::IntExpr, string_expr::StringExpr,
        },
        statements::{
            expression_stmt::ExpressionStmt, let_stmt::LetStatement, var_stmt::VarStatement,
        },
        NodeRef, OpCode,
    },
    Program,
};

use self::{
    environment::Environment,
    objects::{
        boolean::Boolean, error::Error, float::Float, integer::Integer, string::Str, ObjectRef,
    },
};

pub mod environment;
pub mod objects;

#[derive(Default)]
pub struct Evaluator;

impl Evaluator {
    pub fn eval(
        &self,
        node: Option<&NodeRef>,
        enviroment: &mut Environment,
    ) -> Option<Rc<ObjectRef>> {
        if let Some(node) = node {
            match node.get_op_code() {
                OpCode::If => todo!(),
                OpCode::Let => {
                    let let_stmt = node.as_any().downcast_ref::<LetStatement>().unwrap();
                    let name = let_stmt
                        .name
                        .as_any()
                        .downcast_ref::<IdentifierExpr>()
                        .unwrap();
                    let value = Rc::clone(&self.eval(Some(&let_stmt.value), enviroment).unwrap());
                    self.set_immutable(name.value.clone(), Rc::clone(&value), enviroment);
                    Some(value)
                }
                OpCode::Var => {
                    let let_stmt = node.as_any().downcast_ref::<VarStatement>().unwrap();
                    let name = let_stmt
                        .name
                        .as_any()
                        .downcast_ref::<IdentifierExpr>()
                        .unwrap();
                    let value = Rc::clone(&self.eval(Some(&let_stmt.value), enviroment).unwrap());
                    self.set_mutable(name.value.clone(), Rc::clone(&value), enviroment);
                    Some(value)
                }
                OpCode::Ret => todo!(),
                OpCode::Int => {
                    let int_expr = node.as_any().downcast_ref::<IntExpr>().unwrap();
                    Some(Rc::new(Box::new(Integer::new(int_expr.value))))
                }
                OpCode::Call => todo!(),
                OpCode::Bool => {
                    let int_expr = node.as_any().downcast_ref::<BoolExpr>().unwrap();
                    Some(Rc::new(Box::new(Boolean::new(int_expr.value))))
                }
                OpCode::Array => todo!(),
                OpCode::Index => todo!(),
                OpCode::Block => todo!(),
                OpCode::Infix => todo!(),
                OpCode::Float => {
                    let int_expr = node.as_any().downcast_ref::<FloatExpr>().unwrap();
                    Some(Rc::new(Box::new(Float::new(int_expr.value))))
                }
                OpCode::While => todo!(),
                OpCode::Prefix => todo!(),
                OpCode::String => {
                    let int_expr = node.as_any().downcast_ref::<StringExpr>().unwrap();
                    Some(Rc::new(Box::new(Str::new(int_expr.value.clone()))))
                }
                OpCode::Program => {
                    let program = node.as_any().downcast_ref::<Program>().unwrap();
                    Some(self.eval_statements(&program.statements, enviroment))
                }
                OpCode::Function => todo!(),
                OpCode::Expression => {
                    let expr = node.as_any().downcast_ref::<ExpressionStmt>().unwrap();
                    self.eval(Some(&expr.expression), enviroment)
                }
                OpCode::Identifier => {
                    let identifier = node.as_any().downcast_ref::<IdentifierExpr>().unwrap();
                    match enviroment.get_immutabble(&identifier.value) {
                        Some(value) => Some(Rc::clone(&value)),
                        None => match enviroment.get_mutabble(&identifier.value) {
                            Some(value) => Some(Rc::clone(&value)),
                            None => match enviroment.get_function(&identifier.value) {
                                Some(value) => Some(Rc::clone(&value)),
                                None => Some(Rc::new(Box::new(Error::new(format!(
                                    "unknown word {}",
                                    identifier.value
                                ))))),
                            },
                        },
                    }
                }
            }
        } else {
            None
        }
    }

    fn set_immutable(
        &self,
        name: String,
        value: Rc<ObjectRef>,
        enviroment: &mut Environment,
    ) -> Option<Rc<ObjectRef>> {
        enviroment.set_immutable(name, value)
    }

    fn set_mutable(
        &self,
        name: String,
        value: Rc<ObjectRef>,
        enviroment: &mut Environment,
    ) -> Option<Rc<ObjectRef>> {
        enviroment.set_mutable(name, value)
    }

    fn eval_statements(&self, stmts: &[NodeRef], enviroment: &mut Environment) -> Rc<ObjectRef> {
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
