use strum_macros::Display;

use super::expression::ExpressionNode;

pub struct Parameter {
    name: String,
    datatype: Option<String>,
    value: Option<Box<dyn ExpressionNode>>
}

impl Parameter {
    pub fn new(
        name: String, 
        datatype: Option<String>, 
        value: Option<Box<dyn ExpressionNode>>
    ) -> Self {
        Parameter { 
            name, 
            datatype, 
            value
        }
    }

    pub fn new_functional(name: String, datatype: String) -> Self {
        Parameter { 
            name, 
            datatype: Some(datatype), 
            value: None
        }
    }

    pub fn new_calling(name: Option<String>, value: Box<dyn ExpressionNode>) -> Self {
        Parameter {
            name: if name.is_none() {String::new()} else {name.unwrap()},
            datatype: None,
            value: Some(value)
        }
    }
}

pub type Parameters = Vec<Parameter>;

#[derive(Debug, Display, PartialEq, Clone)]
pub enum ParameterType {
    Function,
    Call
}
