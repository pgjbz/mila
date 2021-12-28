use std::any::Any;

use crate::ast::node::{Node, OpCode};

pub struct ExpressionStmt {
    pub expression: Box<dyn Node>,
}

impl ExpressionStmt {
    pub fn new(expr: Box<dyn Node>) -> Self {
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
