use super::context::Context;
use super::tokens::Token;
use std::io::Result;

pub struct Parser {
    tokens: Vec<Token>,
    context: Context,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            context: Context::default(),
            tokens,
        }
    }

    pub fn get_context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn parse () -> Result<()> {
            

        Ok(())
    }
}
