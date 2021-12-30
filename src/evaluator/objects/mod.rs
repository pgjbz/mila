use std::{any::Any, fmt::Display};

pub mod boolean;
pub mod eval_error;
pub mod float;
pub mod function;
pub mod integer;
pub mod string;

pub type ObjectRef = Box<dyn Object>;

pub trait Object: Display {
    fn as_any(&self) -> &dyn Any;
    fn get_type(&self) -> Type;
}

#[derive(PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Float,
    String,
    Error,
    Function,
}
