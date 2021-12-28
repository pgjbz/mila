use crate::ast::node::Node;

pub struct RetStatement {
    pub value: Option<Box<dyn Node>>,
}

impl RetStatement {
    pub fn new(value: Option<Box<dyn Node>>) -> Self {
        Self { value }
    }
}

impl Node for RetStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
