use crate::ast::node::Node;

pub struct VarStatement {
    pub name: Box<dyn Node>,
    pub value: Box<dyn Node>,
}

impl VarStatement {
    pub fn new(name: Box<dyn Node>, value: Box<dyn Node>) -> Self {
        Self { name, value }
    }
}

impl Node for VarStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
