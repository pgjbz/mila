use std::any::Any;

use crate::ast::node::{Node, OpCode};

pub struct FnExpr {
    pub body: Box<dyn Node>,
    pub name: Box<dyn Node>,
    pub parameters: Vec<Box<dyn Node>>,
}

impl FnExpr {
    pub fn new(body: Box<dyn Node>, name: Box<dyn Node>, parameters: Vec<Box<dyn Node>>) -> Self {
        Self {
            body,
            name,
            parameters,
        }
    }
}

impl Node for FnExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Function
    }
}
