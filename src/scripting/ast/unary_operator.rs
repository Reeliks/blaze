use super::{
    expression::ExpressionNode,
    tokens::{TokenSide, TokenType},
};

pub struct UnaryOperatorNode {
    _operator: TokenType,
    _operand: Box<dyn ExpressionNode>,
    _side: TokenSide,
}

impl ExpressionNode for UnaryOperatorNode {
    fn get_type(&self) -> &'static str {
        stringify!(UnaryOperatorNode)
    }
}

impl UnaryOperatorNode {
    pub fn new(operator: TokenType, operand: Box<dyn ExpressionNode>, side: TokenSide) -> Self {
        UnaryOperatorNode {
            _operator: operator,
            _operand: operand,
            _side: side,
        }
    }
}
