pub trait ExpressionNode {
    fn get_type(&self) -> &'static str;
}

