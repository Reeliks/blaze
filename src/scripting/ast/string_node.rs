use super::expression_node::ExpressionNode;

enum StringType {
    Basic,
}

pub struct StringNode {
    value: String,
    string_type: StringType,
}

impl StringNode {
    pub fn new(value: String) -> Self {
        StringNode {
            value,
            string_type: StringType::Basic,
        }
    }
}

impl ExpressionNode for StringNode {}
