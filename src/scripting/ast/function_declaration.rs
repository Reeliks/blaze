use super::expression::ExpressionNode;
use super::parameter::Parameters;

pub struct FunctionDeclarationNode {
    _name: String,
    _datatype: Option<String>,
    _arguments: Parameters,
}

impl FunctionDeclarationNode {
    pub fn new(name: String, datatype: Option<String>, arguments: Parameters) -> Self {
        FunctionDeclarationNode {
            _name: name,
            _datatype: datatype,
            _arguments: arguments,
        }
    }
}

impl ExpressionNode for FunctionDeclarationNode {}
