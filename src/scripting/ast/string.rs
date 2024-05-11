use crate::scripting::ast::expression::ExpressionNode;

enum StringType {
    Basic,
}

pub struct StringNode {
    _value: String,
    _string_type: StringType,
}

impl StringNode {
    pub fn new(value: String) -> Self {
        StringNode {
            _value: value,
            _string_type: StringType::Basic,
        }
    }
}

impl ExpressionNode for StringNode {}
