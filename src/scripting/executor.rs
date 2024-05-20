use super::{ast::body::BodyNode, context::Context};
use std::io::Result;

pub struct Executor<'a> {
    _context: &'a mut Context,
}

impl<'a> Executor<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        Executor { _context: context }
    }

    pub fn execute(self, _nodes: BodyNode) -> Result<()> {
        Ok(())
    }
}
