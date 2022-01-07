use std::{any::Any, fmt::Display};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct CallExpr {
    pub function: NodeRef,
    pub arguments: Vec<NodeRef>,
}

impl CallExpr {
    pub fn new(function: NodeRef, arguments: Vec<NodeRef>) -> Self {
        Self {
            function,
            arguments,
        }
    }
}

impl Node for CallExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Call
    }
}

impl Display for CallExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
