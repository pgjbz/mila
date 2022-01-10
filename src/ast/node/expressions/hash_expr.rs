use std::{any::Any, collections::HashMap, fmt::Display};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct HashExpr {
    pub pairs: HashMap<String, NodeRef>,
}

impl HashExpr {
    pub fn new() -> Self {
        Self {
            pairs: Default::default(),
        }
    }

    pub fn set(&mut self, key: String, value: NodeRef) {
        self.pairs.insert(key, value);
    }
}

impl Node for HashExpr {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Hash
    }
}

impl Display for HashExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for HashExpr {
    fn default() -> Self {
        Self::new()
    }
}
