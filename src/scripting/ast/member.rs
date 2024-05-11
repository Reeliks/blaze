use super::expression::ExpressionNode;

pub struct MemberNode {
    _parent: Box<dyn ExpressionNode>,
    _child: Box<dyn ExpressionNode>,
}

impl ExpressionNode for MemberNode {}

impl MemberNode {
    pub fn new(parent: Box<dyn ExpressionNode>, child: Box<dyn ExpressionNode>) -> Self {
        MemberNode {
            _parent: parent,
            _child: child,
        }
    }
}
