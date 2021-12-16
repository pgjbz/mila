use std::any::Any;

use crate::ast::node::Node;

pub struct PrefixExpr {
    pub operator: String,
    pub right: Box<dyn Node>,
}

impl PrefixExpr {
    pub fn new(operator: String, right: Box<dyn Node>) -> Self {
        Self { operator, right }
    }
}

impl Node for PrefixExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
