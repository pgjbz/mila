use crate::ast::node::Node;

pub struct IfExpr {
    pub condition: Box<dyn Node>,
    pub consequence: Box<dyn Node>,
    pub alternative: Option<Box<dyn Node>>,
    pub el_if: Option<Box<dyn Node>>,
}

impl IfExpr {
    pub fn new(condition: Box<dyn Node>, consequence: Box<dyn Node>) -> Self {
        Self {
            condition,
            consequence,
            alternative: None,
            el_if: None,
        }
    }
}

impl Node for IfExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
