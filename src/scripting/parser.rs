use super::ast::binary_operator_node::{self, BinaryOperatorNode};
use super::ast::boolean_node::BooleanNode;
use super::ast::expression_node::ExpressionNode;
use super::ast::null_node::NullNode;
use super::ast::number_node::NumberNode;
use super::ast::statements_node::StatementsNode;
use super::ast::string_node::StringNode;
use super::ast::variable_node::VariableNode;
use super::context::Context;
use super::tokens::{Token, TokenType, BINARY_OPERATOR_TOKENS, FORMULA_TOKENS};
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

    pub fn get_current_token_and_move(&mut self) -> Token {
        self.context.position += 1;
        self.tokens[self.context.position as usize].clone()
    }

    pub fn parse(&mut self) -> Result<StatementsNode> {
        let mut root = StatementsNode::new();
        while self.context.position < self.tokens.len() as u64 {
            let parsed_expression = self.parse_expression();
            self.require_token_and_move(vec![TokenType::ExpressionEnd])?;
            root.add_node(parsed_expression);
        }
        Ok(root)
    }

    pub fn require_token_and_move(&mut self, expected_tokens: Vec<TokenType>) -> Result<Token> {
        let current_token = self.get_current_token_and_move();
        if expected_tokens.contains(&current_token.token_type) {
            return Ok(current_token);
        }
        let error_message = format!(
            "{} is expected at position {} <= {}:{}:{}",
            expected_tokens[0],
            self.context.position.clone(),
            self.get_context().code_source.clone(),
            self.context.line,
            self.context.position
        );
        Err(io::Error::new(io::ErrorKind::Other, error_message))
    }

    pub fn parse_expression(&mut self) -> Box<dyn ExpressionNode> {
        let current_token = self.get_current_token_and_move();
        match current_token.token_type {
            x if [TokenType::VariableAssign].contains(&x) => {
                let _variable_token = self.require_token_and_move(vec![TokenType::Alphanumeric]);
                let _equals_sign_token = self.require_token_and_move(vec![TokenType::Assign]);
            }
            x if BINARY_OPERATOR_TOKENS.contains(&x) => {}
            _ => {
                self.context.position -= 1;
            }
        }
        Box::new(StatementsNode { nodes: vec![] })
    }

    pub fn parse_formula(mut self) -> Result<Box<dyn ExpressionNode>> {
        let left_node: Box<dyn ExpressionNode>;
        let current_token = self
            .require_token_and_move(FORMULA_TOKENS.to_vec())
            .unwrap();

        match current_token.token_type {
            TokenType::Alphanumeric => {
                left_node = Box::new(VariableNode::new(current_token.value));
            }
            TokenType::CharArray => {
                left_node = Box::new(StringNode::new(current_token.value));
            }
            TokenType::Number => {
                left_node = Box::new(NumberNode::new(current_token.value.parse().unwrap()));
            }
            TokenType::True | TokenType::False => {
                left_node = Box::new(BooleanNode::new(current_token.token_type).unwrap());
            }
            TokenType::Null => {
                left_node = Box::new(NullNode);
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "value expected on assignment <= at {}:{}:{}",
                        self.context.code_source,
                        current_token.line + 1,
                        current_token.position + 1
                    ),
                ))
            }
        };

        match self.get_current_token_and_move()
            .token_type {
            operator if BINARY_OPERATOR_TOKENS.contains(&operator) => {
                let right_node 
                    = self.parse_formula()?;
                let binary_operator_node 
                    = Box::new(
                        BinaryOperatorNode::new(
                        operator, left_node, right_node
                ));
                Ok(binary_operator_node)
            }
            _ => {
                Ok(left_node)
            }
        }
    }
}
