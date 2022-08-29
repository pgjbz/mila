use std::{any::Any, fmt::Display, rc::Rc};

mod array;
mod boolean;
mod built_in;
mod eval_error;
mod float;
mod function;
mod hash;
mod integer;
mod ret;
mod string;

pub use array::*;
pub use boolean::*;
pub use built_in::*;
pub use eval_error::*;
pub use float::*;
pub use function::*;
pub use hash::*;
pub use integer::*;
pub use ret::*;
pub use string::*;

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
