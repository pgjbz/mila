use std::{any::Any, fmt::Display, rc::Rc};

use crate::{ast::node::NodeRef, evaluator::environment::EnvironmentRef};

use super::{Object, Type};

pub struct Function {
    pub body: Rc<NodeRef>,
    pub parameters: Rc<Vec<NodeRef>>,
    pub environment: EnvironmentRef,
}

impl Function {
    pub fn new(
        body: Rc<NodeRef>,
        parameters: Rc<Vec<NodeRef>>,
        environment: EnvironmentRef,
    ) -> Self {
        Self {
            body,
            parameters,
            environment,
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
