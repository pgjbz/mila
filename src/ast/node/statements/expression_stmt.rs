use std::{any::Any, fmt::Display};

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

impl Display for ExpressionStmt {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
