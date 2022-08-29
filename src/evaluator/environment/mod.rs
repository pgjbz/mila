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
        match &self.outer {
            Some(outer) if self.exist_in_outer(&name) => {
                outer.borrow_mut().set_variable(name, value)
            }
            _ => self.variables.insert(name, value),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<ObjectRef> {
        match self.variables.get(name) {
            Some(value) => Some(Rc::clone(value)),
            None => {
                if let Some(env) = &self.outer {
                    env.borrow().get_variable(name)
                } else {
                    None
                }
            }
        }
    }

    fn exist_in_outer(&self, name: &str) -> bool {
        if let Some(outer) = &self.outer {
            return outer.borrow().get_variable(name).is_some();
        }
        false
    }
}
