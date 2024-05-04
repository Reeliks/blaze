use crate::scripting::tokens::TokenType;

use super::expression::ExpressionNode;

pub struct BinaryOperatorNode {
    operator: TokenType,
    left_operand: Box<dyn ExpressionNode>,
    right_operand: Box<dyn ExpressionNode>,
}

impl BinaryOperatorNode {
    pub fn new(
        operator: TokenType,
        left_operand: Box<dyn ExpressionNode>,
        right_operand: Box<dyn ExpressionNode>,
    ) -> Self {
        BinaryOperatorNode {
            operator,
            left_operand,
            right_operand,
        }
    }
}

impl ExpressionNode for BinaryOperatorNode {}
