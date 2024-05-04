use super::expression::ExpressionNode;

pub struct VariableDeclaration {
    name: String,
    datatype: Option<String>,
    value: Box<dyn ExpressionNode>,
}

impl VariableDeclaration {
    pub fn new(name: String, datatype: Option<String>, value: Box<dyn ExpressionNode>) -> Self {
        VariableDeclaration {
            name,
            datatype,
            value,
        }
    }
}

impl ExpressionNode for VariableDeclaration {}
