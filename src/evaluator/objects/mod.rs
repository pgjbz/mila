use std::{any::Any, fmt::Display, rc::Rc};

pub mod array;
pub mod boolean;
pub mod built_in;
pub mod eval_error;
pub mod float;
pub mod function;
pub mod hash;
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
    Hash,
    Float,
    Return,
    String,
    Function,
    BuiltInFn,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print = match self {
            Self::Int => "int",
            Self::Bool => "bool",
            Self::Float => "float",
            Self::String => "string",
            Self::Error => "error",
            Self::Return => "return",
            Self::Function => "function",
            Self::BuiltInFn => "built in function",
            Self::Array => "array",
            Self::Hash => "hash",
        };
        write!(f, "{}", print)
    }
}
