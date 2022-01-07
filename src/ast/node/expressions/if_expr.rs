use std::fmt::Display;

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct IfExpr {
    pub condition: NodeRef,
    pub consequence: NodeRef,
    pub alternative: Option<NodeRef>,
    pub el_if: Option<NodeRef>,
}

impl IfExpr {
    pub fn new(condition: NodeRef, consequence: NodeRef) -> Self {
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

    fn get_op_code(&self) -> OpCode {
        OpCode::If
    }
}

impl Display for IfExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
