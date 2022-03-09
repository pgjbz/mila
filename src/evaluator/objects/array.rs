use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{builtin_map, evaluator::BuiltInMap};

use super::{built_in::BuiltIn, eval_error::EvalError, integer::Integer, Object, ObjectRef, Type};

pub struct Array {
    pub values: RefCell<Vec<ObjectRef>>,
    pub functions: BuiltInMap,
}

//TODO: refactoring array functions
impl Array {
    pub fn new(values: RefCell<Vec<ObjectRef>>) -> Self {
        let functions: BuiltInMap = builtin_map!(
            "push" => Rc::new(BuiltIn::new(push)),
            "replace" => Rc::new(BuiltIn::new(replace)),
            "pop" => Rc::new(BuiltIn::new(pop)),
            "remove" => Rc::new(BuiltIn::new(remove))
        );
        Self { values, functions }
    }
}

fn push(args: &[ObjectRef]) -> ObjectRef {
    if args.len() < 2 {
        return Rc::new(EvalError::new("expected at least one argument".to_string()));
    }
    let mut args_iter = args.iter();
    let arr = args_iter
        .next()
        .unwrap()
        .as_any()
        .downcast_ref::<Array>()
        .unwrap();
    for arg in args_iter {
        arr.values.borrow_mut().push(Rc::clone(arg))
    }
    Rc::clone(&args[0])
}

fn pop(args: &[ObjectRef]) -> ObjectRef {
    if args.len() > 1 {
        return Rc::new(EvalError::new("expected no arguments".to_string()));
    }
    let arr = args
        .iter()
        .next()
        .unwrap()
        .as_any()
        .downcast_ref::<Array>()
        .unwrap();
    if let Some(value) = arr.values.borrow_mut().pop() {
        value
    } else {
        Rc::new(EvalError::new("array is empty".to_string()))
    }
}

fn remove(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 2 {
        return Rc::new(EvalError::new("expected exact one argument".to_string()));
    }
    let mut args_iter = args.iter();
    let arr = args_iter
        .next()
        .unwrap()
        .as_any()
        .downcast_ref::<Array>()
        .unwrap();
    let position = args_iter.next().unwrap();
    if position.get_type() != Type::Int {
        return Rc::new(EvalError::new("index in array only be a int".to_string()));
    }
    let position_cast = position.as_any().downcast_ref::<Integer>().unwrap().value as usize;
    if position_cast >= arr.values.borrow().len() {
        return Rc::new(EvalError::new(format!("invalid position {}", position)));
    }
    arr.values.borrow_mut().remove(position_cast)
}

fn replace(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 3 {
        return Rc::new(EvalError::new("expected two arguments".to_string()));
    }
    let mut args_iter = args.iter();
    let arr = args_iter
        .next()
        .unwrap()
        .as_any()
        .downcast_ref::<Array>()
        .unwrap();
    let position = args_iter.next().unwrap();
    if position.get_type() != Type::Int {
        return Rc::new(EvalError::new("position has to be a int".to_string()));
    }
    let position = position.as_any().downcast_ref::<Integer>().unwrap().value as usize;
    if position > arr.values.borrow().len() {
        return Rc::new(EvalError::new(format!(
            "invalid index to replace {}",
            position
        )));
    }
    let value = args_iter.next().unwrap();
    arr.values.borrow_mut()[position] = Rc::clone(value);
    Rc::clone(&args[0])
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
                .borrow()
                .iter()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        buffer.push(']');

        write!(f, "{}", buffer)
    }
}
