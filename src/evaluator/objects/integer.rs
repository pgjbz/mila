use std::{any::Any, fmt::Display};

use super::{Object, Type};

pub struct Integer {
    pub value: isize,
}

impl Integer {
    pub fn new(value: isize) -> Self {
        Self { value }
    }
}

impl Object for Integer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> Type {
        Type::Int
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
