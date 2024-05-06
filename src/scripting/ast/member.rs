use super::expression::ExpressionNode;

pub struct MemberNode {
    parent: Box<dyn ExpressionNode>,
    child: Box<dyn ExpressionNode>
}

impl ExpressionNode for MemberNode {}

impl MemberNode {
    pub fn new(parent: Box<dyn ExpressionNode>, child: Box<dyn ExpressionNode>) -> Self {
        MemberNode {
            parent, child
        }
    }
}
