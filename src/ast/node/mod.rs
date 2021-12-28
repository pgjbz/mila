pub mod expressions;
pub mod statements;
use std::any::Any;

pub trait Node {
    fn as_any(&self) -> &dyn Any;
    fn get_op_code(&self) -> OpCode;
}

pub enum OpCode {
    If,
    Let,
    Var,
    Ret,
    Int,
    Bool,
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
