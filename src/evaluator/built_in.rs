use std::{
    fs,
    io::{self, Write},
    process,
    rc::Rc,
};

use crate::{downcast_any, evaluator::objects::Str};

use super::objects::{Array, EvalError, Float, Integer, ObjectRef, Type};

pub(super) fn len(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    match first.get_type() {
        Type::Array => {
            let arr_sz = downcast_any!(first => Array).values.borrow().len();
            Rc::new(Integer::new(arr_sz as isize))
        }
        Type::String => {
            let str_sz = downcast_any!(first => Str).value.len();
            Rc::new(Integer::new(str_sz as isize))
        }
        typ => Rc::new(EvalError::new(format!(
            "unsupported operation len of {}",
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
    let _ = std::io::stdout().flush();
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
    let _ = std::io::stderr().flush();
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
    process::exit(downcast_any!(first => Integer).value as i32)
}

pub(super) fn to_int(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    match first.get_type() {
        Type::String => {
            let string = downcast_any!(first => Str);
            if let Ok(value) = string.value.parse::<isize>() {
                Rc::new(Integer::new(value))
            } else {
                Rc::new(EvalError::new(format!(
                    "{} is not parsable to int",
                    string.value
                )))
            }
        }
        Type::Float => {
            let float = downcast_any!(first => Float);
            Rc::new(Integer::new(float.value as isize))
        }
        Type::Int => Rc::clone(first),
        _ => Rc::new(EvalError::new("this type is not parsable".to_string())),
    }
}

pub(super) fn to_float(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new("expected only one argument".to_string()));
    }
    let first = args.first().unwrap();
    match first.get_type() {
        Type::String => {
            let string = downcast_any!(first => Str);
            if let Ok(value) = string.value.parse::<f64>() {
                Rc::new(Float::new(value))
            } else {
                Rc::new(EvalError::new(format!(
                    "{} is not parsable to float",
                    string.value
                )))
            }
        }
        Type::Int => {
            let int = downcast_any!(first => Integer);
            Rc::new(Float::new(int.value as f64))
        }
        Type::Float => Rc::clone(first),
        _ => Rc::new(EvalError::new("this type is not parsable".to_string())),
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

pub(super) fn read_file_as_string(args: &[ObjectRef]) -> ObjectRef {
    if args.len() != 1 {
        return Rc::new(EvalError::new(
            "invalid number of arguments, expected 1 arguments".to_string(),
        ));
    }
    let first = &args[0];
    let path = if first.get_type() != Type::String {
        return Rc::new(EvalError::new("file path has to be a string".to_string()));
    } else {
        &downcast_any!(first => Str).value
    };
    match fs::read_to_string(path) {
        Ok(value) => Rc::new(Str::new(value)),
        Err(e) => Rc::new(EvalError::new(e.to_string())),
    }
}
