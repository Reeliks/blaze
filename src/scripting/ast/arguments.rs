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

pub enum ArgumentType {
    Function,
    Table,
}
