use super::expression_node::ExpressionNode;

pub struct ObjectNode {
    name: String,
}

impl ObjectNode {
    pub fn new(name: String) -> Self {
        ObjectNode { name }
    }
}

impl ExpressionNode for ObjectNode {}
