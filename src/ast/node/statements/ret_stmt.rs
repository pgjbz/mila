use std::any::Any;

use crate::ast::node::{Node, OpCode};

pub struct RetStatement {
    pub value: Option<Box<dyn Node>>,
}

impl RetStatement {
    pub fn new(value: Option<Box<dyn Node>>) -> Self {
        Self { value }
    }
}

impl Node for RetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Ret
    }
}
