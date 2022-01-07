use std::{any::Any, fmt::Display};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct PrefixExpr {
    pub operator: String,
    pub right: NodeRef,
}

impl PrefixExpr {
    pub fn new(operator: String, right: NodeRef) -> Self {
        Self { operator, right }
    }
}

impl Node for PrefixExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Prefix
    }
}

impl Display for PrefixExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
