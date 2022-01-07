use std::{any::Any, fmt::Display, rc::Rc};

use crate::ast::node::{Node, NodeRef, OpCode};

pub struct FnExpr {
    pub body: Rc<NodeRef>,
    pub name: Option<Rc<NodeRef>>,
    pub parameters: Rc<Vec<NodeRef>>,
}

impl FnExpr {
    pub fn new(body: Rc<NodeRef>, name: Option<Rc<NodeRef>>, parameters: Rc<Vec<NodeRef>>) -> Self {
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

impl Display for FnExpr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
