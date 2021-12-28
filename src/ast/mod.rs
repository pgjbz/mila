use self::node::{Node, OpCode};

pub mod node;

pub struct Program {
    pub statements: Vec<Box<dyn Node>>,
    pub errors: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: vec![],
            errors: vec![],
        }
    }

    pub fn push_statements(&mut self, stmt: Box<dyn Node>) {
        self.statements.push(stmt)
    }

    pub fn push_error(&mut self, error: String) {
        self.errors.push(error)
    }
}

impl Node for Program {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_op_code(&self) -> OpCode {
        OpCode::Program
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
