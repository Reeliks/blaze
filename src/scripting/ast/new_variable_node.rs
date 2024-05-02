use super::expression_node::ExpressionNode;

pub struct NewVariableNode {
    name: String,
    datatype: Option<String>,
    value: Box<dyn ExpressionNode>,
}

impl NewVariableNode {
    pub fn new(name: String, datatype: Option<String>, value: Box<dyn ExpressionNode>) -> Self {
        NewVariableNode {
            name,
            datatype,
            value,
        }
    }
}

impl ExpressionNode for NewVariableNode {}
