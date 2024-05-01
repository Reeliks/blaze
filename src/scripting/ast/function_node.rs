use super::expression_node::ExpressionNode;

pub struct FunctionNode {
    name: String,
    datatype: String,
}

impl FunctionNode {
    pub fn new(name: String, datatype: String) -> Self {
        FunctionNode { name, datatype }
    }
}

impl ExpressionNode for FunctionNode {}
