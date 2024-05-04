use crate::scripting::tokens::{TokenSide, TokenType};

use super::expression::ExpressionNode;

pub struct UnaryOperatorNode {
    operator: TokenType,
    operand: Box<dyn ExpressionNode>,
    side: TokenSide
}

impl ExpressionNode for UnaryOperatorNode {}

impl UnaryOperatorNode {
    pub fn new(
        operator: TokenType, 
        operand: Box<dyn ExpressionNode>, 
        side: TokenSide
        ) -> Self {
        UnaryOperatorNode {
            operator,
            operand,
            side
        }
    }
}
