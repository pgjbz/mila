use std::{any::Any, fmt::Display, rc::Rc};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct BlockStatement {
    pub statements: Rc<Vec<NodeRef>>,
}

impl BlockStatement {
    pub fn new(statements: Rc<Vec<NodeRef>>) -> Self {
        Self { statements }
    }
}

impl Node for BlockStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Block
    }
}

impl Default for BlockStatement {
    fn default() -> Self {
        Self::new(Rc::new(vec![]))
    }
}

impl Display for BlockStatement {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
