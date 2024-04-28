use super::ast::node::ExpressionNode;
use super::ast::statements_node::StatementsNode;
use super::context::Context;
use super::tokens::{Token, TokenType};
use std::io::{self, Result};

pub struct Parser {
    tokens: Vec<Token>,
    context: Context,
    nodes: Vec<Box<dyn ExpressionNode>>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            context: Context::default(),
            tokens,
            nodes: vec![]
        }
    }

    pub fn get_context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn parse (mut self) -> Result<Vec<Box<dyn ExpressionNode>>> {
        let mut root = StatementsNode::new();
        loop {
            match self.next_node() {
                Ok(proceed_parsing) => {
                    if !proceed_parsing {
                        return Ok(self.nodes);
                    }
                    let last_node = self.nodes.last().unwrap();
                    self.require_token(vec![TokenType::ExpressionEnd]);
                    root.add_node(**last_node);
                },
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
        }

        Ok(vec![])
    }

    pub fn next_node (&mut self) -> Result<bool> {
        if self.context.position >= self.tokens.len() {
            return Ok(false);
        }
        todo!();
        Ok(false)
    }

    pub fn match_token (&mut self, expected_tokens: &Vec<TokenType>) -> Option<Token> {
        let current_token = 
            &self.tokens[self.context.position as usize]; 

        if expected_tokens.iter().any(|x| x.to_string() == current_token.token_type.to_string()) {
            self.context.position += 1;
            Some(current_token);
        };
        None 
    }

    pub fn require_token (mut self, expected_tokens: Vec<TokenType>) -> Result<Token> {

        let token = self.match_token(&expected_tokens);
        if let Some(token) = token {
            return Ok(token);
        };
        let error_message =
            format!(
                "{} is expected at position {} <= {}:{}:{}",
                 expected_tokens[0].to_string(),
                 self.context.position.clone(),
                 self.get_context().code_source.clone(),
                 self.context.line,
                 self.context.position
            );
        Err(io::Error::new(
                io::ErrorKind::Other, 
                error_message)
        )
    }
}
