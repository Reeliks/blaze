use super::{expression::ExpressionNode, parameter::Parameters};

pub enum CallType {
    Function,
    Position,
    Type,
}

pub struct CallNode {
    callee: Box<dyn ExpressionNode>,
    arguments: Parameters,
    call_type: CallType,
}

impl ExpressionNode for CallNode {}

impl CallNode {
    pub fn new(callee: Box<dyn ExpressionNode>, arguments: Parameters) -> Self {
        CallNode {
            callee,
            arguments,
            call_type: CallType::Function,
        }
    }
}
