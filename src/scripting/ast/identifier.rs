use super::expression::ExpressionNode;

pub struct IdentifierNode {
    _name: String,
    _types: Vec<IdentifierNode>
}

impl ExpressionNode for IdentifierNode {
    fn get_type(&self) -> &'static str {
        stringify!(IdentifierNode)
    }
}

impl IdentifierNode {
    pub fn new(name: String) -> Self {
        IdentifierNode { _name: name, _types: vec![] }
    }
}
