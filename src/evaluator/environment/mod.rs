use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::objects::ObjectRef;

#[derive(Default)]
pub struct Environment {
    pub mutables: HashMap<String, ObjectRef>,
    pub immutables: HashMap<String, ObjectRef>,
    pub functions: HashMap<String, ObjectRef>,
    pub outer: Option<EnvironmentRef>,
}

pub type EnvironmentRef = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new(outer: Option<EnvironmentRef>) -> Self {
        Self {
            outer,
            mutables: Default::default(),
            immutables: Default::default(),
            functions: Default::default(),
        }
    }

    pub fn set_immutable(&mut self, name: String, value: ObjectRef) -> Option<ObjectRef> {
        self.immutables.insert(name, value)
    }

    pub fn set_mutable(&mut self, name: String, value: ObjectRef) -> Option<ObjectRef> {
        if self.exist_in_outer(&name) {
            self.outer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set_mutable(name, value)
        } else {
            self.mutables.insert(name, value)
        }
    }

    pub fn set_function(&mut self, name: String, value: ObjectRef) -> Option<ObjectRef> {
        self.functions.insert(name, value)
    }

    pub fn get_mutabble(&self, name: &str) -> Option<ObjectRef> {
        match self.mutables.get(name) {
            Some(value) => Some(Rc::clone(value)),
            None => match &self.outer {
                Some(ref env) => env.borrow().get_mutabble(name),
                None => None,
            },
        }
    }

    pub fn get_immutabble(&self, name: &str) -> Option<ObjectRef> {
        match self.immutables.get(name) {
            Some(value) => Some(Rc::clone(value)),
            None => match &self.outer {
                Some(ref env) => env.borrow().get_immutabble(name),
                None => None,
            },
        }
    }

    pub fn get_function(&self, name: &str) -> Option<ObjectRef> {
        match self.functions.get(name) {
            Some(value) => Some(Rc::clone(value)),
            None => match &self.outer {
                Some(ref env) => env.borrow().get_function(name),
                None => None,
            },
        }
    }

    fn exist_in_outer(&self, name: &str) -> bool {
        if let Some(ref outer) = self.outer {
            outer.borrow().get_mutabble(name).is_some()
        } else {
            false
        }
    }
}
