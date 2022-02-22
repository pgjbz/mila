use std::{collections::HashMap, fmt::Display};

use super::{Object, ObjectRef, Type};

pub struct HashObj {
    pub pairs: HashMap<String, ObjectRef>,
}

impl Object for HashObj {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type(&self) -> Type {
        super::Hash
    }
}

impl Display for HashObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push('|');
        for (key, value) in self.pairs.iter() {}
        write!(f, "{}", buffer)
    }
}
