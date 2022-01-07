use std::{any::Any, fmt::Display};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct IndexExpr {
    pub left: NodeRef,
    pub index: NodeRef,
}

impl IndexExpr {
    pub fn new(left: NodeRef, index: NodeRef) -> Self {
        Self { left, index }
    }
}

impl Node for IndexExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Index
    }
}

impl Display for IndexExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
