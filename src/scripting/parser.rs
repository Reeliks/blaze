use colored::*;
use rand::seq::SliceRandom;

use super::ast::arguments::{FunctionArgument, PassedArgument};
use super::ast::binary_operator::BinaryOperatorNode;
use super::ast::boolean::BooleanNode;
use super::ast::expression::ExpressionNode;
use super::ast::function_declaration::FunctionDeclarationNode;
use super::ast::unary_operator::UnaryOperatorNode;
use super::ast::variable_declaration::VariableDeclaration;
use super::ast::null::NullNode;
use super::ast::number::NumberNode;
use super::ast::object::ObjectNode;
use super::ast::statements::StatementsNode;
use super::ast::string::StringNode;
use super::context::Context;
use super::tokens::{
    Token, TokenType, BINARY_OPERATOR_TOKENS, FORMULA_TOKENS, UNARY_OPERATOR_TOKENS, VARIABLE_ASSIGNMENT_TOKENS
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

    pub fn parse(&mut self) -> Result<StatementsNode> {
        let mut root = StatementsNode::new();
        let mut add_node = |node: Box<dyn ExpressionNode>| {
            root.add_node(node);
        };
        loop {
            let parsed_expression = self.parse_expression();
            match parsed_expression {
                Ok(Some(..)) => {
                    let parsed_expression = parsed_expression?;
                    add_node(parsed_expression.unwrap());
                    if self.is_position_movable() {
                        self.move_position()?;
                        let semicolon_required = self.require_token(vec![TokenType::ExpressionEnd]);
                        if semicolon_required.is_err() {
                            let err = semicolon_required.unwrap_err();
                            println!("{}", err);
                            return Ok(StatementsNode::new());
                        };
                    };
                    if self.is_position_movable() {
                        self.move_position()?
                    } else {
                        break;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    println!("{}", e);
                    return Ok(StatementsNode::new());
                }
            }
        }
        Ok(root)
    }

    pub fn get_context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn get_current_token(&mut self) -> Result<Token> {
        if self.parser_position < self.tokens.len() as u64 {
            let current_token = self.tokens[self.parser_position as usize].clone();
            // There is no reason to start line and position variables from zero
            // since a parser is not supposed to work with code, but with tokens instead.
            return Ok(current_token);
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "{}{}",
                "FATAL".bright_red(),
                "Attempted to access a non-existent token"
            ),
        ))
    }

    pub fn move_position(&mut self) -> Result<()> {
        let current_token = self.get_current_token()?;
        self.parser_position += 1;
        self.context.line = current_token.line + 1;
        self.context.position = current_token.start + 1;
        Ok(())
    }

    pub fn move_position_back(&mut self) {
        self.parser_position -= 1;
        let current_token = self.get_current_token().unwrap();
        self.context.line = current_token.line + 1;
        self.context.position = current_token.start + 1;
    }

    pub fn is_position_movable(&self) -> bool {
        self.parser_position + 1 < self.tokens.len() as u64
    }

    pub fn require_token(&mut self, expected_tokens: Vec<TokenType>) -> Result<Token> {
        let current_token = self.get_current_token();
        if current_token.is_ok()
            && expected_tokens
                .clone()
                .into_iter()
                .any(|x| current_token.as_ref().unwrap().is_type(x))
        {
            return current_token;
        }
        self.raise_expected_tokens_error(expected_tokens)?;
        current_token
    }

    pub fn raise_expected_tokens_error(&mut self, expected_tokens: Vec<TokenType>) -> Result<()> {
        let error_message = format!(
            "{}{}{}{}",
            "Syntax Error".to_owned().bright_red(),
            ": ",
            &match &expected_tokens[..] {
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
                        "'{}', '{}', '{}', or one of {} other tokens are",
                        shuffled_tokens[0],
                        shuffled_tokens[1],
                        shuffled_tokens[2],
                        shuffled_tokens.len() - 3
                    )
                }
            },
            &format!(
                " expected <-= {}:{}:{}",
                self.context.code_source.clone(),
                self.context.line,
                self.context.position
            )
        );
        Err(io::Error::new(io::ErrorKind::Other, error_message))
    }

    pub fn parse_expression(&mut self) -> Result<Option<Box<dyn ExpressionNode>>> {
        if self.get_current_token().is_err() {
            return Ok(None);
        };
        let current_token = self.get_current_token()?;
        self.move_position()?;
        match current_token.token_type {
            x if VARIABLE_ASSIGNMENT_TOKENS.contains(&x) => {
                let variable_token
                    = self.require_token(vec![TokenType::Alphanumeric])?;
                self.move_position()?;
                let datatype = self.parse_datatype()?;
                if datatype.is_some() {
                    self.move_position()?
                };
                self.require_token(vec![TokenType::Assign])?;
                self.move_position()?;
                Ok(Some(
                    Box::new(
                        VariableDeclaration::new(
                        variable_token.value,
                        datatype,
                        self.parse_formula()?
                    )))
                )
            }
            x if FORMULA_TOKENS.contains(&x) => {
                self.move_position_back();
                let formula_node = self.parse_formula()?;
                Ok(Some(formula_node))
            }
            TokenType::Function => {
                let name_token =
                    self.require_token(vec![TokenType::Alphanumeric])?;
                self.move_position()?;
                let arguments
                    = self.parse_function_arguments()?;
                self.move_position()?;
                let datatype = self.parse_datatype()?.clone();
                if datatype.is_none() {
                    self.move_position_back();
                }
                Ok(Some(
                    Box::new(
                        FunctionDeclarationNode::new(
                        name_token.value,
                        datatype,
                        arguments
                    )))
                )
            }
            TokenType::ExpressionEnd => {
                Ok(self.parse_expression()?)
            }
            _ => {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                        format!(
                            "{}: {} hasn't been implemented yet or is not being considered in this context <-= at {}:{}:{}",
                            "Syntax Error".bright_red(),
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
        let current_token = self.get_current_token();
        if current_token.is_ok() && current_token.unwrap().is_type(TokenType::Colon) {
            self.move_position()?;
            let datatype_token = self.require_token(vec![TokenType::Alphanumeric])?;
            return Ok(Some(datatype_token.value));
        };
        Ok(None)
    }

    pub fn parse_function_arguments(&mut self) -> Result<Vec<FunctionArgument>> {
        let mut arguments: Vec<FunctionArgument> = vec![];
        self.require_token(vec![TokenType::LPar])?;
        loop {
            if !self.is_position_movable() {
                self.raise_expected_tokens_error(vec![TokenType::RPar])?;
            }
            self.move_position()?;
            let mut current_token = self.get_current_token().unwrap();

            if !arguments.is_empty() && current_token.is_type(TokenType::Comma) {
                self.move_position()?;
                current_token = self.require_token(vec![TokenType::Alphanumeric])?;
            };

            match current_token.token_type {
                TokenType::Alphanumeric => {
                    self.move_position()?;
                    let argument_datatype = self.parse_datatype()?;
                    if argument_datatype.is_none() {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
<<<<<<< HEAD
                                "{}{}",
                                "Syntax Error".bright_red(),
                                format!(
                                    ": Argument type is expected <-= {}:{}:{}",
                                    self.context.code_source,
                                    self.context.line,
                                    self.context.position
                                )
=======
                                "{}: Argument type is expected <-= {}:{}:{}",
                                "Syntax Error".bright_red(),
                                self.context.code_source,
                                self.context.line,
                                self.context.position
>>>>>>> c42b73369f374f001167d8f82bdb45f6033ceffd
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
<<<<<<< HEAD
                            "{}{}",
                            "Syntax Error".bright_red(),
                            format!(
                                ": Argument expected <-= {}:{}:{}",
                                self.context.code_source, self.context.line, self.context.position
                            )
=======
                            "{}: Argument expected <-= {}:{}:{}",
                            "Syntax Error".bright_red(),
                            self.context.code_source,
                            self.context.line,
                            self.context.position
>>>>>>> c42b73369f374f001167d8f82bdb45f6033ceffd
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
        let first_token = self.require_token(FORMULA_TOKENS.to_vec())?;
        let left_operand: Box<dyn ExpressionNode> = match first_token.token_type {
            TokenType::Alphanumeric => Box::new(ObjectNode::new(first_token.clone().value)),
            TokenType::CharArray => Box::new(StringNode::new(first_token.clone().value)),
            TokenType::Number => Box::new(NumberNode::new(first_token.value.parse().unwrap())),
            TokenType::Null => Box::new(NullNode),
            TokenType::True | TokenType::False => {
<<<<<<< HEAD
                Box::new(BooleanNode::new(first_token.token_type.clone()).unwrap())
=======
                Box::new(BooleanNode::new(current_token.token_type).unwrap())
>>>>>>> c42b73369f374f001167d8f82bdb45f6033ceffd
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
<<<<<<< HEAD
                        "{}{}",
                        "Syntax Error".bright_red(),
                        format!(
                            ": Value expected on assignment <-= at {}:{}:{}",
                            self.context.code_source,
                            first_token.line + 1,
                            first_token.start + 1
                        )
=======
                        "{}: Value expected on assignment <-= at {}:{}:{}",
                        "Syntax Error".bright_red(),
                        self.context.code_source,
                        current_token.line + 1,
                        current_token.position + 1
>>>>>>> c42b73369f374f001167d8f82bdb45f6033ceffd
                    ),
                ));
            }
        };
        if !self.is_position_movable() {
            return Ok(left_operand);
        }
        self.move_position()?;
        let second_token = self.get_current_token()?;
        match second_token.token_type {
            operator if BINARY_OPERATOR_TOKENS.contains(&operator) => {
                self.move_position()?;
                let right_operand = self.parse_formula()?;
                let binary_operator_node =
                    Box::new(BinaryOperatorNode::new(operator, left_operand, right_operand));
                Ok(binary_operator_node)
            }
            operator if UNARY_OPERATOR_TOKENS.contains(&operator) => {
                if !first_token.is_type(TokenType::Alphanumeric) {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "{}: Unary operator works only with objects <-= at {}:{}:{}",
                            "Syntax Error".bright_red(),
                            self.context.code_source,
                            first_token.line + 1,
                            first_token.start + 1
                        )
                    ));
                };
                let unary_operator_node =
                    Box::new(UnaryOperatorNode::new(operator, left_operand));
                Ok(unary_operator_node)
            }
            _ => {
                self.move_position_back();
                Ok(left_operand)
            }
        }
    }
}
