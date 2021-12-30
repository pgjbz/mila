use std::{process, rc::Rc};

use crate::{
    ast::{
        node::{
            expressions::{
                bool_expr::BoolExpr, float_expr::FloatExpr, fn_expr::FnExpr,
                identifier_expr::IdentifierExpr, int_expr::IntExpr, string_expr::StringExpr,
            },
            statements::{
                expression_stmt::ExpressionStmt, let_stmt::LetStatement, var_stmt::VarStatement,
            },
            NodeRef, OpCode,
        },
        Program,
    },
    evaluator::objects::Type,
};

use self::{
    environment::Environment,
    objects::{
        boolean::Boolean, eval_error::EvalError, float::Float, function::Function,
        integer::Integer, string::Str, ObjectRef,
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
                OpCode::Function => {
                    let function_expr = node.as_any().downcast_ref::<FnExpr>().unwrap();
                    let body = Rc::clone(&function_expr.body);
                    let parameters = Rc::clone(&function_expr.parameters);
                    let name = Rc::clone(&function_expr.name);
                    let name_string = name
                        .as_any()
                        .downcast_ref::<IdentifierExpr>()
                        .unwrap()
                        .value
                        .clone();
                    let function: Rc<ObjectRef> =
                        Rc::new(Box::new(Function::new(body, name, parameters)));
                    self.set_function(name_string, Rc::clone(&function), enviroment);
                    Some(function)
                }
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
                                None => Some(Rc::new(Box::new(EvalError::new(format!(
                                    "unknown word '{}'",
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

    fn set_function(
        &self,
        name: String,
        value: Rc<ObjectRef>,
        enviroment: &mut Environment,
    ) -> Option<Rc<ObjectRef>> {
        enviroment.set_function(name, value)
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

    fn _is_error(&self, to_check: &Option<Rc<ObjectRef>>) -> bool {
        match to_check {
            Some(check) if check.get_type() == Type::Error => {
                let eval_error = check.as_any().downcast_ref::<EvalError>().unwrap();
                eprintln!("{}", eval_error.message);
                true
            }
            _ => false,
        }
    }
}
