use std::{collections::HashMap, rc::Rc};

use super::objects::ObjectRef;

#[derive(Default)]
pub struct Environment {
    pub mutables: HashMap<String, Rc<ObjectRef>>,
    pub immutables: HashMap<String, Rc<ObjectRef>>,
    pub functions: HashMap<String, Rc<ObjectRef>>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn set_immutable(&mut self, name: String, value: Rc<ObjectRef>) -> Option<Rc<ObjectRef>> {
        self.immutables.insert(name, value)
    }

    pub fn set_mutable(&mut self, name: String, value: Rc<ObjectRef>) -> Option<Rc<ObjectRef>> {
        self.mutables.insert(name, value)
    }

    pub fn set_function(&mut self, name: String, value: Rc<ObjectRef>) -> Option<Rc<ObjectRef>> {
        self.functions.insert(name, value)
    }

    pub fn get_mutabble(&self, name: &str) -> Option<Rc<ObjectRef>> {
        match self.mutables.get(name) {
            Some(value) => Some(Rc::clone(value)),
            None => match &self.outer {
                Some(ref env) => env.get_mutabble(name),
                None => None,
            },
        }
    }

    pub fn get_immutabble(&self, name: &str) -> Option<Rc<ObjectRef>> {
        match self.immutables.get(name) {
            Some(value) => Some(Rc::clone(value)),
            None => match &self.outer {
                Some(ref env) => env.get_immutabble(name),
                None => None,
            },
        }
    }

    pub fn get_function(&self, name: &str) -> Option<Rc<ObjectRef>> {
        match self.functions.get(name) {
            Some(value) => Some(Rc::clone(value)),
            None => match &self.outer {
                Some(ref env) => env.get_function(name),
                None => None,
            },
        }
    }
}
