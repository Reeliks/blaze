use super::expression_node::ExpressionNode;

pub struct VariableNode {
    name: String,
}

impl VariableNode {
    pub fn new(name: String) -> Self {
        VariableNode { name }
    }
}

impl ExpressionNode for VariableNode {}
