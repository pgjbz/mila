use crate::ast::node::Node;

pub struct ExpressionStmt {
    pub expression: Box<dyn Node>,
}

impl ExpressionStmt {
    pub fn new(expr: Box<dyn Node>) -> Self {
        Self { expression: expr }
    }
}

impl Node for ExpressionStmt {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
