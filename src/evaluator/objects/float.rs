use std::{any::Any, fmt::Display};

use super::{Object, Type};

pub struct Float {
    pub value: f64,
}

impl Float {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Object for Float {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> Type {
        Type::Int
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
