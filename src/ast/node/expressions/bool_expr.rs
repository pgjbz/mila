use crate::ast::node::Node;

pub struct BoolExpr {
    pub value: bool,
}

impl BoolExpr {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Node for BoolExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
