use std::io;

use crate::scripting::tokens::TokenType;

use super::expression::ExpressionNode;

pub struct BooleanNode {
    state: bool,
}

impl BooleanNode {
    pub fn new(token_type: TokenType) -> Result<Self, io::Error> {
        match token_type {
            TokenType::True => Ok(BooleanNode { state: true }),
            TokenType::False => Ok(BooleanNode { state: false }),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid value for a boolean node",
            )),
        }
    }
}

impl ExpressionNode for BooleanNode {}

impl Default for BooleanNode {
    fn default() -> Self {
        Self::new(TokenType::False).unwrap()
    }
}
