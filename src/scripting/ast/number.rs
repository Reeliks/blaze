use super::expression::ExpressionNode;

pub struct NumberNode {
    value: f64,
}

impl NumberNode {
    pub fn new(value: f64) -> Self {
        NumberNode { value }
    }
}

impl ExpressionNode for NumberNode {}
