use std::fmt::Display;

use crate::ast::node::{Node, OpCode};

pub struct BoolExpr {
    pub value: bool,
}

impl BoolExpr {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Node for BoolExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Bool
    }
}

impl Display for BoolExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
