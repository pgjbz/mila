use std::any::Any;

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
