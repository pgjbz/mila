use std::{any::Any, fmt::Display};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct LetStatement {
    pub name: NodeRef,
    pub value: NodeRef,
}

impl LetStatement {
    pub fn new(name: NodeRef, value: NodeRef) -> Self {
        Self { name, value }
    }
}

impl Node for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Let
    }
}

impl Display for LetStatement {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
