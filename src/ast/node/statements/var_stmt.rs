use std::{any::Any, fmt::Display};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct VarStatement {
    pub name: NodeRef,
    pub value: NodeRef,
}

impl VarStatement {
    pub fn new(name: NodeRef, value: NodeRef) -> Self {
        Self { name, value }
    }
}

impl Node for VarStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Var
    }
}

impl Display for VarStatement {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
