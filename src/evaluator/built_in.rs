use std::rc::Rc;

use crate::evaluator::objects::string::Str;

use super::objects::ObjectRef;

pub(super) fn len(args: &[ObjectRef]) -> ObjectRef {
    todo!()
}

pub(super) fn puts(args: &[ObjectRef]) -> ObjectRef {
    let mut buffer = String::with_capacity(50);
    for arg in args {
        buffer.push_str(&arg.to_string())
    }
    println!("{}", buffer);
    Rc::new(Str::new(buffer))
}
