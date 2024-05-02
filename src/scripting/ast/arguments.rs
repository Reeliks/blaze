use super::expression_node::ExpressionNode;

pub trait Argument {}

pub struct FunctionArgument {
    name: String,
    datatype: String,
}

impl Argument for FunctionArgument {}

impl FunctionArgument {
    pub fn new(name: String, datatype: String) -> Self {
        FunctionArgument { name, datatype }
    }
}

pub struct PassedArgument {
    name: Option<String>,
    value: Box<dyn ExpressionNode>
}

impl PassedArgument {
    pub fn new(name: Option<String>, value: Box<dyn ExpressionNode>) -> Self {
        PassedArgument {
            name, value
        } 
    }
}

impl Argument for PassedArgument {}

