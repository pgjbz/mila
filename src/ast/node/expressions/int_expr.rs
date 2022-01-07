use std::fmt::Display;

use crate::ast::node::{Node, OpCode};

pub struct IntExpr {
    pub value: isize,
}

impl IntExpr {
    pub fn new(value: isize) -> Self {
        Self { value }
    }
}

impl Node for IntExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Int
    }
}

impl Display for IntExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
