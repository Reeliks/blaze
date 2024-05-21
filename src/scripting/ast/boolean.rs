use std::io;

use super::{expression::ExpressionNode, tokens::TokenType};

pub struct BooleanNode {
    _state: bool,
}

impl BooleanNode {
    pub fn new(token_type: TokenType) -> Result<Self, io::Error> {
        match token_type {
            TokenType::True => Ok(BooleanNode { _state: true }),
            TokenType::False => Ok(BooleanNode { _state: false }),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid value for a boolean node",
            )),
        }
    }
}

impl ExpressionNode for BooleanNode {
    fn get_type(&self) -> &'static str {
        stringify!(BooleanNode)
    }
}

impl Default for BooleanNode {
    fn default() -> Self {
        Self::new(TokenType::False).unwrap()
    }
}
