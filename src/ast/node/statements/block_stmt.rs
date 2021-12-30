use std::rc::Rc;

use crate::ast::node::{Node, NodeRef};

pub struct BlockStatement {
    pub statements: Vec<Rc<NodeRef>>,
}

impl BlockStatement {
    pub fn new() -> Self {
        Self {
            statements: Vec::with_capacity(5),
        }
    }

    pub fn push_stmt(&mut self, stmt: Rc<NodeRef>) {
        self.statements.push(stmt)
    }
}

impl Node for BlockStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_op_code(&self) -> crate::ast::node::OpCode {
        todo!()
    }
}

impl Default for BlockStatement {
    fn default() -> Self {
        Self::new()
    }
}
