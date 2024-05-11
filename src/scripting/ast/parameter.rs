use strum_macros::Display;

use super::expression::ExpressionNode;

pub struct Parameter {
    _name: String,
    _datatype: Option<String>,
    _value: Option<Box<dyn ExpressionNode>>, // Value field can either store a transmitted value
                                             // or contain a default value for functions, plans, and tables.
}

impl Parameter {
    pub fn new(
        name: String,
        datatype: Option<String>,
        value: Option<Box<dyn ExpressionNode>>,
    ) -> Self {
        Parameter {
            _name: name,
            _datatype: datatype,
            _value: value,
        }
    }

    pub fn new_functional(name: String, datatype: String) -> Self {
        Parameter {
            _name: name,
            _datatype: Some(datatype),
            _value: None,
        }
    }

    pub fn new_calling(name: Option<String>, value: Box<dyn ExpressionNode>) -> Self {
        Parameter {
            _name: name.unwrap_or_default(),
            _datatype: None,
            _value: Some(value),
        }
    }
}

pub type Parameters = Vec<Parameter>;

#[derive(Debug, Display, PartialEq, Clone)]
pub enum ParameterType {
    Function,
    Call,
}
