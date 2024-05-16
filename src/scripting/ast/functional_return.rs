use super::expression::ExpressionNode;

pub struct FunctionalReturnNode {
    _value: Option<Box<dyn ExpressionNode>>
}

impl ExpressionNode for FunctionalReturnNode {
    fn get_type(&self) -> &'static str {
        stringify!(FunctionalReturnNode)
    }
}

impl FunctionalReturnNode {
    pub fn new(value: Option<Box<dyn ExpressionNode>>) -> Self {
        FunctionalReturnNode {
            _value: value
        }
    }
}
