use std::collections::HashMap;

use super::objects::ObjectRef;

#[derive(Default)]
pub struct Environment {
    pub mutables: HashMap<String, ObjectRef>,
    pub imutables: HashMap<String, ObjectRef>,
    pub functions: HashMap<String, ObjectRef>,
    pub outer: Option<Box<Environment>>,
}
