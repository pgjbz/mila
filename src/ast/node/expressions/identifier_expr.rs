use std::fmt::Display;

use crate::ast::node::{Node, OpCode};

pub struct IdentifierExpr {
    pub value: String,
}

impl IdentifierExpr {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for IdentifierExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Identifier
    }
}

impl Display for IdentifierExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
