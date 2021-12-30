use std::{any::Any, fmt::Display, rc::Rc};

use crate::ast::node::NodeRef;

use super::{Object, Type};

pub struct Function {
    pub body: Rc<NodeRef>,
    pub name: Rc<NodeRef>,
    pub parameters: Rc<Vec<NodeRef>>,
}

impl Function {
    pub fn new(body: Rc<NodeRef>, name: Rc<NodeRef>, parameters: Rc<Vec<NodeRef>>) -> Self {
        Self {
            body,
            name,
            parameters,
        }
    }
}

impl Object for Function {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> Type {
        Type::Function
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function(...){{...}}")
    }
}
