use super::ast::binary_operator_node::BinaryOperatorNode;
use super::ast::boolean_node::BooleanNode;
use super::ast::expression_node::ExpressionNode;
use super::ast::function_node::FunctionNode;
use super::ast::new_variable_node::NewVariableNode;
use super::ast::null_node::NullNode;
use super::ast::number_node::NumberNode;
use super::ast::object_node::ObjectNode;
use super::ast::statements_node::StatementsNode;
use super::ast::string_node::StringNode;
use super::ast::variable_node::VariableNode;
use super::context::Context;
use super::tokens::{
    Token, TokenType, BINARY_OPERATOR_TOKENS, FORMULA_TOKENS, VARIABLE_ASSIGNMENT_TOKENS,
};
use std::io::{self, Result};

pub struct Parser {
    tokens: Vec<Token>,
    context: Context,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            context: Context::default(),
            tokens,
        }
    }

    pub fn get_context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn get_current_token_and_move(&mut self) -> Option<Token> {
        if self.context.position < self.tokens.len() as u64 {
            self.context.position += 1;
            return Some(self.tokens[self.context.position as usize].clone());
        }
        None
    }

    pub fn require_token_and_move(&mut self, expected_tokens: Vec<TokenType>) -> Result<Token> {
        let current_token = self.get_current_token_and_move().unwrap();
        if expected_tokens.contains(&current_token.token_type) {
            return Ok(current_token);
        }
        let error_message = format!(
            "{} is expected at position {} <= {}:{}:{}",
            expected_tokens[0].regex_str(), // Should be replaced with expectred_tokens[0].name(),
            // but it's not implemented yet
            self.context.position.clone(),
            self.context.code_source.clone(),
            self.context.line,
            self.context.position
        );
        Err(io::Error::new(io::ErrorKind::Other, error_message))
    }

    pub fn parse(&mut self) -> Result<StatementsNode> {
        let mut root = StatementsNode::new();
        while let Ok(Some(parsed_expression)) = self.parse_expression() {
            root.add_node(parsed_expression);
        }
        Ok(root)
    }

    pub fn parse_expression(&mut self) -> Result<Option<Box<dyn ExpressionNode>>> {
        let current_token = self.get_current_token_and_move();
        if current_token.is_none() {
            return Ok(None);
        };
        let current_token = current_token.unwrap();

        match current_token.token_type {
            // NewVariableNode: fin my_variable: int = 5;
            // VariableNode: my_variable = 5;
            x if VARIABLE_ASSIGNMENT_TOKENS.contains(&x) => {
                let datatype: String = self.parse_datatype().unwrap();
                let variable_token
                    = self.require_token_and_move(vec![TokenType::Alphanumeric])
                    .unwrap();
                self.require_token_and_move(vec![TokenType::Assign])
                    .unwrap();
                Ok(Some(
                    Box::new(
                        NewVariableNode::new(
                        variable_token.value,
                        datatype,
                        self.parse_formula().unwrap()
                    ))
                ))
            }
            TokenType::Alphanumeric => {
                let _equals_sign_token
                    = self.require_token_and_move(vec![TokenType::Assign]);
                Ok(Some(
                    Box::new(
                        VariableNode::new(
                        current_token.value,
                        self.parse_formula().unwrap(),
                    ))
                ))
            }
            TokenType::Function => {
                let name_token =
                    self.require_token_and_move(vec![TokenType::Alphanumeric])
                    .unwrap();
                self.require_token_and_move(vec![TokenType::LPar])
                    .unwrap();
                self.require_token_and_move(vec![TokenType::RPar])
                    .unwrap();
                let datatype = self.parse_datatype().unwrap();
                Ok(Some(
                    Box::new(
                        FunctionNode::new(
                        name_token.value,
                        datatype
                    ))
                ))
            }
            _ => {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{} isn't recognized at parsing stage; perhaps it's not implemented yet <= {}:{}:{}",
                        current_token.token_type,
                        self.context.code_source,
                        current_token.line + 1,
                        current_token.position + 1
                    )
                ))
            }
        }
    }

    pub fn parse_datatype(&mut self) -> Result<String> {
        let mut datatype: String = "unknown".to_string();
        let next_token = self.get_current_token_and_move().unwrap();
        if next_token.is_type(TokenType::Colon) {
            let datatype_token = self
                .require_token_and_move(vec![TokenType::Alphanumeric])
                .unwrap();
            datatype = datatype_token.value;
        } else {
            self.context.position -= 1;
        }
        Ok(datatype)
    }

    pub fn parse_formula(&mut self) -> Result<Box<dyn ExpressionNode>> {
        let current_token = self
            .require_token_and_move(FORMULA_TOKENS.to_vec())
            .unwrap();

        let left_node: Box<dyn ExpressionNode> = match current_token.token_type {
            TokenType::Alphanumeric => Box::new(ObjectNode::new(current_token.value)),
            TokenType::CharArray => Box::new(StringNode::new(current_token.value)),
            TokenType::Number => Box::new(NumberNode::new(current_token.value.parse().unwrap())),
            TokenType::True | TokenType::False => {
                Box::new(BooleanNode::new(current_token.token_type).unwrap())
            }
            TokenType::Null => Box::new(NullNode),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "value expected on assignment <= at {}:{}:{}",
                        self.context.code_source,
                        current_token.line + 1,
                        current_token.position + 1
                    ),
                ));
            }
        };

        match self.get_current_token_and_move().unwrap().token_type {
            operator if BINARY_OPERATOR_TOKENS.contains(&operator) => {
                let right_node = self.parse_formula()?;
                let binary_operator_node =
                    Box::new(BinaryOperatorNode::new(operator, left_node, right_node));
                Ok(binary_operator_node)
            }
            _ => Ok(left_node),
        }
    }
}
