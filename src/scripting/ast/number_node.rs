use super::expression_node::ExpressionNode;

pub struct NumberNode {
    value: i64,
}

impl NumberNode {
    pub fn new(value: i64) -> Self {
        NumberNode { value }
    }
}

impl ExpressionNode for NumberNode {}
