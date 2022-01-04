use std::{any::Any, fmt::Display, rc::Rc};

pub mod array;
pub mod boolean;
pub mod eval_error;
pub mod float;
pub mod function;
pub mod integer;
pub mod ret;
pub mod string;

pub type ObjectRef = Rc<dyn Object>;

pub trait Object: Display {
    fn as_any(&self) -> &dyn Any;
    fn get_type(&self) -> Type;
}

#[derive(PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Array,
    Error,
    Float,
    String,
    Function,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print = match self {
            Self::Int => "int",
            Self::Bool => "bool",
            Self::Float => "float",
            Self::String => "string",
            Self::Error => "error",
            Self::Function => "function",
            Self::Array => "array",
        };
        write!(f, "{}", print)
    }
}
