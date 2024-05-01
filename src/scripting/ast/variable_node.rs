use super::expression_node::ExpressionNode;

pub struct VariableNode {
    name: String,
    value: Box<dyn ExpressionNode>,
}

impl VariableNode {
    pub fn new(name: String, value: Box<dyn ExpressionNode>) -> Self {
        VariableNode { name, value }
    }
}

impl ExpressionNode for VariableNode {}
