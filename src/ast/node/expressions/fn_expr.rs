use std::{any::Any, rc::Rc};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct FnExpr {
    pub body: Rc<NodeRef>,
    pub name: Rc<NodeRef>,
    pub parameters: Rc<Vec<NodeRef>>,
}

impl FnExpr {
    pub fn new(body: Rc<NodeRef>, name: Rc<NodeRef>, parameters: Rc<Vec<NodeRef>>) -> Self {
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
