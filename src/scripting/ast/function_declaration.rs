use super::expression::ExpressionNode;
use super::parameter::Parameters;

pub struct FunctionDeclarationNode {
    _name: String,
    _datatype: Option<String>,
    _generics: Vec<String>,
    _arguments: Parameters,
}

impl FunctionDeclarationNode {
    pub fn new(name: String, datatype: Option<String>, arguments: Parameters) -> Self {
        FunctionDeclarationNode {
            _name: name,
            _datatype: datatype,
            _generics: vec![],
            _arguments: arguments,
        }
    }
}

impl ExpressionNode for FunctionDeclarationNode {
    fn get_type(&self) -> &'static str {
        stringify!(FunctionDeclarationNode)
    }
}
