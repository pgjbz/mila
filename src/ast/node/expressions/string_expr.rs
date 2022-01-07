use std::fmt::Display;

use crate::ast::node::{Node, OpCode};

pub struct StringExpr {
    pub value: String,
}

impl StringExpr {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for StringExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::String
    }
}

impl Display for StringExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
