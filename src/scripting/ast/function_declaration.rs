use super::arguments::FunctionArgument;
use super::expression::ExpressionNode;

pub struct FunctionDeclarationNode {
    name: String,
    datatype: Option<String>,
    arguments: Vec<FunctionArgument>,
}

impl FunctionDeclarationNode {
    pub fn new(name: String, datatype: Option<String>, arguments: Vec<FunctionArgument>) -> Self {
        FunctionDeclarationNode {
            name,
            datatype,
            arguments,
        }
    }
}

impl ExpressionNode for FunctionDeclarationNode {}
