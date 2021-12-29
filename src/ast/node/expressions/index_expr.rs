use std::any::Any;

use crate::ast::node::{Node, OpCode};

pub struct IndexExpr {
    pub left: Box<dyn Node>,
    pub index: Box<dyn Node>,
}

impl IndexExpr {
    pub fn new(left: Box<dyn Node>, index: Box<dyn Node>) -> Self {
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
