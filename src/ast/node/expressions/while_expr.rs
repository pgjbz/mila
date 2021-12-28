use crate::ast::node::{Node, OpCode};

pub struct WhileExpr {
    pub condition: Box<dyn Node>,
    pub consequence: Box<dyn Node>,
}

impl WhileExpr {
    pub fn new(condition: Box<dyn Node>, consequence: Box<dyn Node>) -> Self {
        Self {
            condition,
            consequence,
        }
    }
}

impl Node for WhileExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::While
    }
}
