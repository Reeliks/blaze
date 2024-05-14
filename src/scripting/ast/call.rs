use super::{expression::ExpressionNode, parameter::Parameters};

pub enum CallType {
    Function,
    Position,
    Type,
}

pub struct CallNode {
    _callee: Box<dyn ExpressionNode>,
    _arguments: Parameters,
    _call_type: CallType,
}

impl ExpressionNode for CallNode {
    fn get_type(&self) -> &'static str {
        stringify!(CallNode)
    }
}

impl CallNode {
    pub fn new(callee: Box<dyn ExpressionNode>, arguments: Parameters) -> Self {
        CallNode {
            _callee: callee,
            _arguments: arguments,
            _call_type: CallType::Function,
        }
    }
}
