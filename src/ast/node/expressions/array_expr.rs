use std::{any::Any, fmt::Display};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct ArrayExpr {
    pub values: Vec<NodeRef>,
}

impl ArrayExpr {
    pub fn new(values: Vec<NodeRef>) -> Self {
        Self { values }
    }
}

impl Node for ArrayExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Array
    }
}

impl Display for ArrayExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
