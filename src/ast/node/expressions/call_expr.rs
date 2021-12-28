use std::any::Any;

use crate::ast::node::{Node, OpCode};

pub struct CallExpr {
    pub function: Box<dyn Node>,
    pub arguments: Vec<Box<dyn Node>>,
}

impl CallExpr {
    pub fn new(function: Box<dyn Node>, arguments: Vec<Box<dyn Node>>) -> Self {
        Self {
            function,
            arguments
        }
    }
}

impl Node for CallExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Call
    }
}