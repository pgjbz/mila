use crate::ast::node::Node;

pub struct BlockStatement {
    pub statements: Vec<Box<dyn Node>>,
}

impl BlockStatement {
    pub fn new() -> Self {
        Self {
            statements: Vec::with_capacity(5),
        }
    }

    pub fn push_stmt(&mut self, stmt: Box<dyn Node>) {
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
