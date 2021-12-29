use std::any::Any;

use crate::ast::node::{Node, OpCode};

pub struct ArrayExpr {
    pub values: Vec<Box<dyn Node>>,
}

impl ArrayExpr {
    pub fn new(values: Vec<Box<dyn Node>>) -> Self {
        Self { values }
    }
}

impl Node for ArrayExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Array
    }
}
