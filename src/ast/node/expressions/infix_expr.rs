use std::{any::Any, fmt::Display};

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

impl Display for InfixExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
