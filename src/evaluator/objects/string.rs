use std::{any::Any, collections::HashMap, fmt::Display, rc::Rc};

use crate::{builtin_map, downcast_any, evaluator::BuiltInMap};

use super::{built_in::BuiltIn, eval_error::EvalError, Object, ObjectRef, Type};

pub struct Str {
    pub value: String,
    pub functions: BuiltInMap,
}

impl Str {
    pub fn new(value: String) -> Self {
        let functions: BuiltInMap = builtin_map!(
        "trim" => Rc::new(BuiltIn::new(trim)));
        Self { value, functions }
    }
}

impl Object for Str {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> Type {
        Type::String
    }
}

pub(super) fn trim(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new(format!(
            "invalid number of arguments, expected 0 got {}",
            args.len() - 1
        )));
    }
    let first = args.first().unwrap();
    Rc::new(Str::new(
        downcast_any!(first => Str).value.trim().to_string(),
    ))
}

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
