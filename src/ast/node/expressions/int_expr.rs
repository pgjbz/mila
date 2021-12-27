use crate::ast::node::Node;

pub struct IntExpr {
    pub value: isize,
}

impl IntExpr {
    pub fn new(value: isize) -> Self {
        Self { value }
    }
}

impl Node for IntExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
