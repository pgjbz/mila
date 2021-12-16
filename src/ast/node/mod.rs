pub mod expressions;
pub mod statements;
use std::any::Any;

pub trait Node {
    fn as_any(&self) -> &dyn Any;
}
