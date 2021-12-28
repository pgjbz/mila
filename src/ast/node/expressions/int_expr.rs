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
