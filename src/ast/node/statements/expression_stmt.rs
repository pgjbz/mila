use std::any::Any;

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct ExpressionStmt {
    pub expression: NodeRef,
}

impl ExpressionStmt {
    pub fn new(expr: NodeRef) -> Self {
        Self { expression: expr }
    }
}

impl Node for ExpressionStmt {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Expression
    }
}
