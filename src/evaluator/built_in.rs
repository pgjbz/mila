use std::{process, rc::Rc};

use crate::evaluator::objects::string::Str;

use super::objects::{array::Array, eval_error::EvalError, integer::Integer, ObjectRef, Type};

pub(super) fn len(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(Str::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    match first.get_type() {
        Type::Array => {
            let arr_sz = first.as_any().downcast_ref::<Array>().unwrap().values.len();
            Rc::new(Integer::new(arr_sz as isize))
        }
        Type::String => {
            let str_sz = first.as_any().downcast_ref::<Str>().unwrap().value.len();
            Rc::new(Integer::new(str_sz as isize))
        }
        typ => Rc::new(EvalError::new(format!(
            "unsuported operation len of {}",
            typ
        ))),
    }
}

pub(super) fn puts(args: &[ObjectRef]) -> ObjectRef {
    let mut buffer = String::with_capacity(50);
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    print!("{}", buffer);
    Rc::new(Str::new(buffer))
}

pub(super) fn putsln(args: &[ObjectRef]) -> ObjectRef {
    let mut buffer = String::with_capacity(50);
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    println!("{}", buffer);
    Rc::new(Str::new(buffer))
}

pub(super) fn eputs(args: &[ObjectRef]) -> ObjectRef {
    let mut buffer = String::with_capacity(50);
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    eprint!("{}", buffer);
    Rc::new(Str::new(buffer))
}

pub(super) fn eputsln(args: &[ObjectRef]) -> ObjectRef {
    let mut buffer = String::with_capacity(50);
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    eprint!("{}", buffer);
    Rc::new(Str::new(buffer))
}

pub(super) fn exit(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(Str::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    if first.get_type() != Type::Int {
        return Rc::new(Str::new("only use int values".to_string()));
    }
    process::exit(first.as_any().downcast_ref::<Integer>().unwrap().value as i32)
}
