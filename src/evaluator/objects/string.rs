use std::{any::Any, fmt::Display};

use super::{Object, Type};

pub struct Str {
    pub value: String,
}

impl Str {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Object for Str {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> Type {
        Type::Int
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
