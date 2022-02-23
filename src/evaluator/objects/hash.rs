use std::{collections::HashMap, fmt::Display};

use super::{Object, ObjectRef, Type};

pub struct HashObj {
    pub pairs: HashMap<String, ObjectRef>,
}

impl HashObj {
    pub fn new() -> Self {
        Self {
            pairs: Default::default(),
        }
    }

    pub fn put(&mut self, key: String, value: ObjectRef) {
        self.pairs.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&ObjectRef> {
        self.pairs.get(key)
    }
}

impl Object for HashObj {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type(&self) -> Type {
        Type::Hash
    }
}

impl Default for HashObj {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for HashObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str("|\n");
        for (key, value) in self.pairs.iter() {
            buffer.push('\t');
            buffer.push_str(key);
            buffer.push_str(": ");
            buffer.push_str(&value.to_string());
            buffer.push_str(", \n");
        }
        buffer.push('|');
        write!(f, "{}", buffer)
    }
}
