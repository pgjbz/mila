use std::any::Any;

use crate::ast::node::Node;

pub struct InfixExpr {
    pub operator: String,
    pub right: Box<dyn Node>,
    pub left: Box<dyn Node>,
}

impl InfixExpr {
    pub fn new(operator: String, right: Box<dyn Node>, left: Box<dyn Node>) -> Self {
        Self {
            operator,
            right,
            left,
        }
    }
}

impl Node for InfixExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
