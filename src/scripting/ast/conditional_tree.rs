use super::expression::ExpressionNode;

pub type Conditions = Vec<(Box<dyn ExpressionNode>, Box<dyn ExpressionNode>)>;

pub struct ConditionalTreeNode {
    _conditions: Conditions,
    _default: Option<Box<dyn ExpressionNode>>,
}

impl ExpressionNode for ConditionalTreeNode {
    fn get_type(&self) -> &'static str {
        stringify!(ConditionalTreeNode)
    }
}

impl ConditionalTreeNode {
    pub fn new(conditions: Conditions, default: Option<Box<dyn ExpressionNode>>) -> Self {
        ConditionalTreeNode {
            _conditions: conditions,
            _default: default,
        }
    }
}
