use super::expression::ExpressionNode;

pub struct NumberNode {
    _value: f64,
}

impl NumberNode {
    pub fn new(value: f64) -> Self {
        NumberNode { _value: value }
    }
}

impl ExpressionNode for NumberNode {}
