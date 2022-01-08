use std::{io, process, rc::Rc};

use crate::evaluator::objects::string::Str;

use super::objects::{
    array::Array, eval_error::EvalError, float::Float, integer::Integer, ObjectRef, Type,
};

pub(super) fn len(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    match first.get_type() {
        Type::Array => {
            let arr_sz = first
                .as_any()
                .downcast_ref::<Array>()
                .unwrap()
                .values
                .borrow()
                .len();
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
        return Rc::new(EvalError::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    if first.get_type() != Type::Int {
        return Rc::new(EvalError::new("only use int values".to_string()));
    }
    process::exit(first.as_any().downcast_ref::<Integer>().unwrap().value as i32)
}

pub(super) fn to_int(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    match first.get_type() {
        Type::String => {
            let string = first.as_any().downcast_ref::<Str>().unwrap();
            if let Ok(value) = string.value.parse::<isize>() {
                Rc::new(Integer::new(value))
            } else {
                Rc::new(EvalError::new(format!(
                    "{} is not parseable to int",
                    string.value
                )))
            }
        }
        Type::Float => {
            let float = first.as_any().downcast_ref::<Float>().unwrap();
            Rc::new(Integer::new(float.value as isize))
        }
        Type::Int => Rc::clone(first),
        _ => Rc::new(EvalError::new("this type is not parseable".to_string())),
    }
}

pub(super) fn to_float(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    match first.get_type() {
        Type::String => {
            let string = first.as_any().downcast_ref::<Str>().unwrap();
            if let Ok(value) = string.value.parse::<f64>() {
                Rc::new(Float::new(value))
            } else {
                Rc::new(EvalError::new(format!(
                    "{} is not parseable to float",
                    string.value
                )))
            }
        }
        Type::Int => {
            let int = first.as_any().downcast_ref::<Integer>().unwrap();
            Rc::new(Float::new(int.value as f64))
        }
        Type::Float => Rc::clone(first),
        _ => Rc::new(EvalError::new("this type is not parseable".to_string())),
    }
}

pub(super) fn to_string(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new("expected only one argument".to_string()));
    }
    Rc::new(Str::new(format!("{}", args.first().unwrap())))
}

pub(super) fn read(args: &[ObjectRef]) -> ObjectRef {
    if !args.is_empty() {
        return Rc::new(EvalError::new(
            "invalid number of arguments, expected 0 arguments".to_string(),
        ));
    }
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    Rc::new(Str::new(buffer))
}

pub(super) fn trim(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    if first.get_type() != Type::String {
        return Rc::new(EvalError::new("only use string values".to_string()));
    }
    Rc::new(Str::new(
        first
            .as_any()
            .downcast_ref::<Str>()
            .unwrap()
            .value
            .trim()
            .to_string(),
    ))
}
