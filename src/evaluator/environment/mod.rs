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
}
