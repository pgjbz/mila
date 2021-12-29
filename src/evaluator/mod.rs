use crate::ast::node::NodeRef;

use self::{environment::Enviroment, objects::ObjectRef};

pub mod environment;
pub mod objects;

pub struct Evaluator;

impl Evaluator {
    pub fn eval(&self, node: Option<NodeRef>, _enviroment: &mut Enviroment) -> Option<ObjectRef> {
        if let Some(_node) = node {
            todo!()
        } else {
            None
        }
    }
}
