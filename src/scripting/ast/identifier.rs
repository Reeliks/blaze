use super::expression::ExpressionNode;

pub struct IdentifierNode {
    name: String,
}

impl ExpressionNode for IdentifierNode {}

impl IdentifierNode {
    pub fn new(name: String) -> Self {
        IdentifierNode { name }
    }
}
