use super::body::BodyNode;
use super::expression::ExpressionNode;
use super::parameter::Parameters;

pub struct FunctionDeclarationNode {
    _name: String,
    _datatype: Option<String>,
    _generics: Vec<String>,
    _arguments: Parameters,
    _body: Option<BodyNode>,
}

impl FunctionDeclarationNode {
    pub fn new(
        name: String,
        datatype: Option<String>,
        arguments: Parameters,
        body: Option<BodyNode>,
    ) -> Self {
        FunctionDeclarationNode {
            _name: name,
            _datatype: datatype,
            _generics: vec![],
            _arguments: arguments,
            _body: body,
        }
    }
}

impl ExpressionNode for FunctionDeclarationNode {
    fn get_type(&self) -> &'static str {
        stringify!(FunctionDeclarationNode)
    }
}
