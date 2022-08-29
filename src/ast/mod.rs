use std::fmt::Display;

use self::node::{Node, NodeRef, OpCode};

pub mod node;

pub use node::*;

pub struct Program {
    pub statements: Vec<NodeRef>,
    pub errors: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: vec![],
            errors: vec![],
        }
    }

    pub fn push_statements(&mut self, stmt: NodeRef) {
        self.statements.push(stmt)
    }

    pub fn push_error(&mut self, error: String) {
        self.errors.push(error)
    }
}

impl Node for Program {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Program
    }
}

impl Display for Program {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
