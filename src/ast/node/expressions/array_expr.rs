use std::any::Any;

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct ArrayExpr {
    pub values: Vec<NodeRef>,
}

impl ArrayExpr {
    pub fn new(values: Vec<NodeRef>) -> Self {
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
