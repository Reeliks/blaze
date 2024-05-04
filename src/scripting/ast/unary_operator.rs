use crate::scripting::tokens::TokenType;

use super::expression::ExpressionNode;

pub struct UnaryOperatorNode {
    operator: TokenType,
    operand: Box<dyn ExpressionNode>,
}

impl ExpressionNode for UnaryOperatorNode {}

impl UnaryOperatorNode {
    pub fn new(operator: TokenType, operand: Box<dyn ExpressionNode>) -> Self {
        UnaryOperatorNode { operator, operand }
    }
}
