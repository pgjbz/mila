pub mod expressions;
pub mod statements;
use std::{any::Any, fmt::Display};

pub type NodeRef = Box<dyn Node>;
pub trait Node: Display {
    fn as_any(&self) -> &dyn Any;
    fn get_op_code(&self) -> OpCode;
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    If,
    Let,
    Var,
    Ret,
    Int,
    Call,
    Bool,
    Array,
    Index,
    Block,
    Infix,
    Float,
    While,
    Prefix,
    String,
    Program,
    Function,
    Expression,
    Identifier,
}
