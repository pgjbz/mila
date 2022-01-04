use std::{any::Any, fmt::Display};

use super::{ObjectRef, Object, Type};


pub type BuildInFn = fn(&[ObjectRef]) -> ObjectRef;


pub struct BuiltIn {
    pub function: BuildInFn,
}

impl BuiltIn {
    pub fn new(function: BuildInFn) -> Self {
        Self { function }
    }
}

impl Object for BuiltIn {
    fn get_type(&self) -> Type {
        Type::BuiltInFn
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "build in function")
    }
}