use super::expression::ExpressionNode;

pub struct IdentifierNode {
    _name: String,
}

impl ExpressionNode for IdentifierNode {}

impl IdentifierNode {
    pub fn new(name: String) -> Self {
        IdentifierNode { _name: name }
    }
}
