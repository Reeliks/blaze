use super::expression::ExpressionNode;

pub struct NullNode;

impl ExpressionNode for NullNode {
    fn get_type(&self) -> &'static str {
        stringify!(NullNode)
    }
}
