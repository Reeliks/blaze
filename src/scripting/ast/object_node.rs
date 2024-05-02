use super::expression_node::ExpressionNode;

pub struct ObjectNode {
    name: String,
}

impl ExpressionNode for ObjectNode {}

impl ObjectNode {
    pub fn new(name: String) -> Self {
        ObjectNode { name }
    }
}
