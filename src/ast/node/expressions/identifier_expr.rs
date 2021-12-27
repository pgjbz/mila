use crate::ast::node::Node;

pub struct IdentifierExpr {
    pub value: String,
}

impl IdentifierExpr {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for IdentifierExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
