use std::{any::Any, fmt::Display, collections::HashMap};

use super::{Object, ObjectRef, Type};

pub struct Array {
    pub values: Vec<ObjectRef>,
    pub functions: HashMap<String, ObjectRef>,
}

impl Array {
    pub fn new(values: Vec<ObjectRef>) -> Self {
        Self { values, functions: Default::default() }
    }
}

impl Object for Array {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> Type {
        Type::Array
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push('[');
        buffer.push_str(
            &self
                .values
                .iter()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        buffer.push('[');

        write!(f, "{}", buffer)
    }
}
