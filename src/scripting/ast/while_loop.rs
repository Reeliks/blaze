use super::{body::BodyNode, expression::ExpressionNode};

pub struct WhileLoopNode {
    _condition: Box<dyn ExpressionNode>,
    _body: BodyNode
}

impl ExpressionNode for WhileLoopNode {
    fn get_type(&self) -> &'static str {
        stringify!(WhileLoopNode)
    }
}

impl WhileLoopNode {
    pub fn new(condition: Box<dyn ExpressionNode>, body: BodyNode) -> Self {
        WhileLoopNode {
            _condition: condition,
            _body: body
        }
    }
}
