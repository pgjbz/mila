use std::any::Any;

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct FnExpr {
    pub body: NodeRef,
    pub name: NodeRef,
    pub parameters: Vec<NodeRef>,
}

impl FnExpr {
    pub fn new(body: NodeRef, name: NodeRef, parameters: Vec<NodeRef>) -> Self {
        Self {
            body,
            name,
            parameters,
        }
    }
}

impl Node for FnExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Function
    }
}
