use std::{fmt::Display, rc::Rc};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Location {
    line: usize,
    column: usize,
    file: Rc<String>,
}

impl Location {
    pub fn new(line: usize, column: usize, file: Rc<String>) -> Self {
        Self { line, column, file }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}
