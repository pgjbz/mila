use crate::ast::node::{Node, OpCode};

pub struct FloatExpr {
    pub value: f64,
}

impl FloatExpr {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Node for FloatExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Float
    }
}
