use crate::scripting::tokens::TokenType;

use super::expression_node::ExpressionNode;

pub struct BinaryOperatorNode {
    operator: TokenType,
    left_node: Box<dyn ExpressionNode>,
    right_node: Box<dyn ExpressionNode>,
}

impl BinaryOperatorNode {
    pub fn new(
        operator: TokenType,
        left_node: Box<dyn ExpressionNode>,
        right_node: Box<dyn ExpressionNode>,
    ) -> Self {
        BinaryOperatorNode {
            operator,
            left_node,
            right_node,
        }
    }
}

impl ExpressionNode for BinaryOperatorNode {}
