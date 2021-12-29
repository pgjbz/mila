use crate::ast::node::{Node, NodeRef, OpCode};

pub struct WhileExpr {
    pub condition: NodeRef,
    pub consequence: NodeRef,
}

impl WhileExpr {
    pub fn new(condition: NodeRef, consequence: NodeRef) -> Self {
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
