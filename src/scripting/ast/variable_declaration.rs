use super::expression::ExpressionNode;

pub struct VariableDeclaration {
    _name: String,
    _datatype: Option<String>,
    _value: Option<Box<dyn ExpressionNode>>,
}

impl VariableDeclaration {
    pub fn new(
        name: String,
        datatype: Option<String>,
        value: Option<Box<dyn ExpressionNode>>,
    ) -> Self {
        VariableDeclaration {
            _name: name,
            _datatype: datatype,
            _value: value,
        }
    }
}

impl ExpressionNode for VariableDeclaration {
    fn get_type(&self) -> &'static str {
        stringify!(VariableDeclaration)
    }
}
