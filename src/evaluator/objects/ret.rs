use std::{any::Any, fmt::Display};

use super::{Object, ObjectRef, Type};

pub struct Ret {
    pub val: ObjectRef,
}

impl Ret {
    pub fn new(val: ObjectRef) -> Self {
        Self { val }
    }
}

impl Object for Ret {
    fn get_type(&self) -> Type {
        self.val.get_type()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Ret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
