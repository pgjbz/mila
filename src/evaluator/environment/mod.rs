use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::objects::ObjectRef;

#[derive(Default)]
pub struct Environment {
    pub variables: HashMap<String, ObjectRef>,
    pub outer: Option<EnvironmentRef>,
}

pub type EnvironmentRef = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new(outer: Option<EnvironmentRef>) -> Self {
        Self {
            outer,
            variables: Default::default(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: ObjectRef) -> Option<ObjectRef> {
        if self.exist_in_outer(&name) {
            self.outer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set_variable(name, value)
        } else {
            self.variables.insert(name, value)
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<ObjectRef> {
        if let Some(value) = self.variables.get(name) {
            Some(Rc::clone(value))
        } else if let Some(env) = &self.outer {
            env.borrow_mut().get_variable(name)
        } else {
            None
        }
    }

    fn exist_in_outer(&self, name: &str) -> bool {
        if let Some(ref outer) = self.outer {
            outer.borrow().get_variable(name).is_some()
        } else {
            false
        }
    }
}
