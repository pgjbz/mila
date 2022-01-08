use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use super::{built_in::BuiltIn, eval_error::EvalError, integer::Integer, Object, ObjectRef, Type};

pub struct Array {
    pub values: RefCell<Vec<ObjectRef>>,
    pub functions: HashMap<String, ObjectRef>,
}

//TODO: refactoring array functions
impl Array {
    pub fn new(values: RefCell<Vec<ObjectRef>>) -> Self {
        let mut functions: HashMap<String, ObjectRef> = HashMap::new();
        functions.insert("push".to_string(), Rc::new(BuiltIn::new(push)));
        functions.insert("replace".to_string(), Rc::new(BuiltIn::new(replace)));
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
