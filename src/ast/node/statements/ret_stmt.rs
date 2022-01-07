use std::{any::Any, fmt::Display};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct RetStatement {
    pub value: Option<NodeRef>,
}

impl RetStatement {
    pub fn new(value: Option<NodeRef>) -> Self {
        Self { value }
    }
}

impl Node for RetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Ret
    }
}

impl Display for RetStatement {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
