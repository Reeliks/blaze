use super::{expression::ExpressionNode, tokens::TokenType};

pub struct BinaryOperatorNode {
    _operator: TokenType,
    _left_operand: Box<dyn ExpressionNode>,
    _right_operand: Box<dyn ExpressionNode>,
}

impl BinaryOperatorNode {
    pub fn new(
        operator: TokenType,
        left_operand: Box<dyn ExpressionNode>,
        right_operand: Box<dyn ExpressionNode>,
    ) -> Self {
        BinaryOperatorNode {
            _operator: operator,
            _left_operand: left_operand,
            _right_operand: right_operand,
        }
    }
}

impl ExpressionNode for BinaryOperatorNode {
    fn get_type(&self) -> &'static str {
        stringify!(BinaryOperatorNode)
    }
}
