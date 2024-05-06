use super::parameter::Parameters;
use super::expression::ExpressionNode;

pub struct FunctionDeclarationNode {
    name: String,
    datatype: Option<String>,
    arguments: Parameters,
}

impl FunctionDeclarationNode {
    pub fn new(name: String, datatype: Option<String>, arguments: Parameters) -> Self {
        FunctionDeclarationNode {
            name,
            datatype,
            arguments,
        }
    }
}

impl ExpressionNode for FunctionDeclarationNode {}
