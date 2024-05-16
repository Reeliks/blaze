use super::expression::ExpressionNode;

pub enum LoopControlType {
    Continue,
    Break
}

pub struct LoopControlNode {
    _control_type: LoopControlType
}

impl ExpressionNode for LoopControlNode {
    fn get_type(&self) -> &'static str {
        stringify!(LoopControlNode)
    }
}

impl LoopControlNode {
    pub fn new(control_type: LoopControlType) -> Self {
        LoopControlNode {
            _control_type: control_type
        }
    }
}
