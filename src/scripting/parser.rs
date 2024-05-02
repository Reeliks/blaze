use rand::seq::SliceRandom;

use super::ast::arguments::{FunctionArgument, PassedArgument};
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
    parser_position: u64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            context: Context::default(),
            tokens,
            parser_position: 0,
        }
    }

    pub fn get_context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn get_current_token(&mut self) -> Result<Token> {
        if self.parser_position < self.tokens.len() as u64 {
            let current_token = self.tokens[self.parser_position as usize].clone();
            // There is no reason to start line and position variables from zero
            // since a parser is not supposed to work with code, but with tokens instead.
            return Ok(current_token)
        }
        Err(io::Error::new(
            io::ErrorKind::Other, 
            "Stack overflow"
        ).into())
    }

    pub fn move_position(&mut self) -> Result<()> {
        let current_token = self.get_current_token().unwrap();
        self.parser_position += 1; 
        self.context.line = current_token.line + 1;
        self.context.position = current_token.position + 1;
        Ok(())
    }

    pub fn require_token(&mut self, expected_tokens: Vec<TokenType>) -> Result<Token> {
        let current_token = self.get_current_token();
        if !current_token.is_err() 
            && expected_tokens.contains(&current_token.as_ref().unwrap().token_type) {
            return Ok(current_token?);
        }
        self.raise_expected_tokens_error(expected_tokens)?;
        current_token
    }
    
    pub fn raise_expected_tokens_error(&mut self, expected_tokens: Vec<TokenType>) -> Result<()> {
        let error_message = "Syntax Error: ".to_owned()
            + &match &expected_tokens[..] {
                [] => "void".to_string(),
                [first] => format!("'{}' is", first),
                [first, second] => format!("'{}' or '{}' are", first, second),
                [first, second, third] => format!("'{}', '{}', or '{}' are", first, second, third),
                [first, second, third, fourth] => {
                    format!("{}, {}, {}, or {} are", first, second, third, fourth)
                }
                _ => {
                    let mut shuffled_tokens = expected_tokens.clone();
                    shuffled_tokens.shuffle(&mut rand::thread_rng());
                    format!(
                        "'{}', '{}', '{}', or one of '{}' other tokens are",
                        shuffled_tokens[0],
                        shuffled_tokens[1],
                        shuffled_tokens[2],
                        shuffled_tokens.len() - 3
                    )
                }
            }
            + &format!(
                " expected <= {}:{}:{}",
                self.context.code_source.clone(),
                self.context.line,
                self.context.position
            );
        Err(io::Error::new(io::ErrorKind::Other, error_message))
    }

    pub fn parse(&mut self) -> Result<StatementsNode> {
        let mut root = StatementsNode::new();
        while let Ok(Some(parsed_expression)) = self.parse_expression() {
            if self.parser_position < self.tokens.len() as u64 {
                self.require_token(vec![TokenType::ExpressionEnd])?;
            } 
            root.add_node(parsed_expression);
            break;
        }
        Ok(root)
    }

    pub fn parse_expression(&mut self) -> Result<Option<Box<dyn ExpressionNode>>> {
        let current_token = self.get_current_token();
        if !current_token.is_ok() {
            return Ok(None);
        };

        let current_token = current_token.unwrap();
        self.move_position()?;
        match current_token.token_type {
            // NewVariableNode: fin my_variable: int = 5;
            // VariableNode: my_variable = 5;
            x if VARIABLE_ASSIGNMENT_TOKENS.contains(&x) => {
                let variable_token
                    = self.require_token(vec![TokenType::Alphanumeric])
                    .unwrap();
                self.move_position()?;
                let datatype = self.parse_datatype().unwrap();
                if datatype.is_some() {
                    self.move_position()?
                };
                self.require_token(vec![TokenType::Assign])
                    .unwrap();
                self.move_position()?;
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
                    = self.require_token(vec![TokenType::Assign]);
                self.move_position()?;
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
                    self.require_token(vec![TokenType::Alphanumeric])
                    .unwrap();
                self.move_position()?;
                let arguments 
                    = self.parse_function_arguments()?;
                self.move_position()?;
                let datatype = self.parse_datatype();
                if datatype.is_err() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "return type of {} is not specified <-= at {}:{}:{}",
                            name_token.value,
                            self.context.code_source,
                            self.context.line,
                            self.context.position
                        )
                    ));
                };
                Ok(Some(
                    Box::new(
                        FunctionNode::new(
                        name_token.value,
                        datatype.unwrap(),
                        arguments
                    ))
                ))
            }
            _ => {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{} isn't recognized at parsing stage; perhaps it's not implemented yet <-= at {}:{}:{}",
                        current_token.token_type,
                        self.context.code_source,
                        self.context.line,
                        self.context.position
                    )
                ))
            }
        }
    }

    pub fn parse_datatype(&mut self) -> Result<Option<String>> {
        let next_token = self.get_current_token();
        if next_token.is_err() {
            self.raise_expected_tokens_error(vec![TokenType::Colon])?;
        }
        if next_token?.is_type(TokenType::Colon) {
            self.move_position()?;
            let datatype_token = self
                .require_token(vec![TokenType::Alphanumeric])
                .unwrap();
            return Ok(Some(datatype_token.value))
        };
        Ok(None)
    }

    pub fn parse_function_arguments(&mut self) -> Result<Vec<FunctionArgument>> {
        let mut arguments: Vec<FunctionArgument> = vec![];
        self.require_token(vec![TokenType::LPar]).unwrap();
        loop {
            self.move_position()?;
            let current_token = self.get_current_token().unwrap();
            if !arguments.is_empty() && current_token.is_type(TokenType::Comma) 
            {
                self.move_position()?;
                self.require_token(vec![TokenType::Alphanumeric])?;
                println!("suceed");
            };
            match current_token.token_type {
                TokenType::Alphanumeric => {
                    self.move_position()?;
                    let argument_datatype = self.parse_datatype().unwrap();
                    if argument_datatype.is_none() {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "Argument type is expected <= {}:{}:{}",
                                self.context.code_source, self.context.line, self.context.position
                            ),
                        ));
                    };
                    arguments.push(FunctionArgument::new(
                        current_token.value,
                        argument_datatype.unwrap(),
                    )); 
                }
                TokenType::RPar => {
                    break;
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "Argument expected <= {}:{}:{}",
                            self.context.code_source, self.context.line, self.context.position
                        ),
                    ));
                }
            }
        }
        Ok(arguments)
    }
    
    pub fn parse_passed_arguments(&mut self) -> Result<Vec<PassedArgument>> {
        todo!()
    }

    pub fn parse_formula(&mut self) -> Result<Box<dyn ExpressionNode>> {
        let current_token = self
            .require_token(FORMULA_TOKENS.to_vec())
            .unwrap();
        let left_node: Box<dyn ExpressionNode> = match current_token.token_type {
            TokenType::Alphanumeric => {
                // self.move_position()?;
                // if self.get_current_token()?.is_type(TokenType::LPar) {
                //      
                // };
                Box::new(ObjectNode::new(current_token.value))
            },
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
        match self.get_current_token().unwrap().token_type {
            operator if BINARY_OPERATOR_TOKENS.contains(&operator) => {
                self.move_position()?;
                let right_node = self.parse_formula()?;
                let binary_operator_node =
                    Box::new(BinaryOperatorNode::new(operator, left_node, right_node));
                Ok(binary_operator_node)
            }
            _ => Ok(left_node),
        }
    }
}
