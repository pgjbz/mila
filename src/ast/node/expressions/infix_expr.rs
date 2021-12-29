use std::any::Any;

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct InfixExpr {
    pub operator: String,
    pub right: NodeRef,
    pub left: NodeRef,
}

impl InfixExpr {
    pub fn new(operator: String, right: NodeRef, left: NodeRef) -> Self {
        Self {
            operator,
            right,
            left,
        }
    }
}

impl Node for InfixExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Infix
    }
}
