use super::arguments::FunctionArgument;
use super::expression_node::ExpressionNode;

pub struct FunctionNode {
    name: String,
    datatype: Option<String>,
    arguments: Vec<FunctionArgument>,
}

impl FunctionNode {
    pub fn new(name: String, datatype: Option<String>, arguments: Vec<FunctionArgument>) -> Self {
        FunctionNode {
            name,
            datatype,
            arguments,
        }
    }
}

impl ExpressionNode for FunctionNode {}
