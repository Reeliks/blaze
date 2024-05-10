use colored::*; use rand::seq::SliceRandom; use super::ast::call::CallNode;
use super::ast::member::MemberNode;
use super::ast::parameter::{Parameter, ParameterType, Parameters};
use super::ast::binary_operator::BinaryOperatorNode;
use super::ast::boolean::BooleanNode;
use super::ast::expression::ExpressionNode;
use super::ast::function_declaration::FunctionDeclarationNode;
use super::ast::unary_operator::UnaryOperatorNode;
use super::ast::variable_declaration::VariableDeclaration;
use super::ast::null::NullNode;
use super::ast::number::NumberNode;
use super::ast::identifier::IdentifierNode;
use super::ast::body::BodyNode;
use super::ast::string::StringNode;
use super::context::Context;
use super::tokens::{
    Token, 
    TokenSide, 
    TokenType, 
    BINARY_OPERATOR_TOKENS, 
    FORMULA_TOKENS, 
    UNARY_OPERATOR_TOKENS, 
    VARIABLE_ASSIGNMENT_TOKENS
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

    pub fn parse(&mut self) -> Result<BodyNode> {
        let mut root = BodyNode::new();
        let mut add_node
            = |node: Box<dyn ExpressionNode>| {
            root.add_node(node);
        };
        loop {
            let parsed_expression
                = self.parse_expression();
            match parsed_expression {
                Ok(Some(..)) => {
                    let parsed_expression = parsed_expression.unwrap();
                    add_node(parsed_expression.unwrap());
                    if self.move_if_position_is_movable() {
                        let semicolon_required
                            = self.require_token(vec![TokenType::ExpressionEnd]);
                        if semicolon_required.is_err() {
                            let err = semicolon_required.unwrap_err();
                            println!("{}", err);
                            return Ok(BodyNode::new());
                        };
                    }
                    else {
                        break;
                    };
                }
                Ok(None) => break,
                Err(e) => {
                    println!("{}", e);
                    return Ok(BodyNode::new());
                }
            }
        }
        Ok(root)
    }

    pub fn get_context(&mut self) -> &mut Context {
        &mut self.context
    }

    fn get_current_token(&mut self) -> Result<Token> {
        if self.parser_position < self.tokens.len() as u64 {
            let current_token = self.tokens[self.parser_position as usize].clone();
            return Ok(current_token);
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "{}: {}",
                "FATAL".red(),
                "Attempted to access a non-existent token"
            ),
        ))
    }

    fn move_position(&mut self) -> Token {
        let current_token = self.get_current_token().unwrap();
        self.parser_position += 1;
        // There is no reason to start line and position variables from zero
        // since a parser is not supposed to work with code, but with tokens instead.
        self.context.line = current_token.line + 1;
        self.context.position = current_token.start + 1;
        current_token
    }

    fn move_position_back(&mut self) {
        self.parser_position -= 1;
        let current_token = self.get_current_token().unwrap();
        self.context.line = current_token.line + 1;
        self.context.position = current_token.start + 1;
    }

    fn is_position_movable(&self) -> bool {
        self.parser_position + 1 < self.tokens.len() as u64
    }

    fn move_if_position_is_movable(&mut self) -> bool {
        if !self.is_position_movable() {
            return false;
        }
        self.move_position();
        true
    }

    fn move_if_next_token_is(&mut self, desired_types: Vec<TokenType>) -> bool {
        if !self.move_if_position_is_movable() {
            return false;
        };
        if desired_types
            .into_iter()
            .any(|x| x == self.get_current_token().unwrap().token_type) {
            return true;
        };
        self.move_position_back();
        false

    }

    fn require_token(&mut self, expected_tokens: Vec<TokenType>) -> Result<Token> {
        let current_token = self.get_current_token();
        if current_token.is_ok()
            && expected_tokens
                .clone()
                .into_iter()
                .any(|x| current_token.as_ref().unwrap().is_type(x))
        {
            return current_token;
        }
        self.raise_expected_tokens_error(expected_tokens).unwrap();
        current_token
    }

    fn raise_expected_tokens_error(&mut self, expected_tokens: Vec<TokenType>) -> Result<()> {
        let mut shuffled_tokens = expected_tokens.clone();
        shuffled_tokens.shuffle(&mut rand::thread_rng());
        let error_message = format!(
            "{}{}{}{}",
            "Syntax Error".bright_red(),
            ": ",
            &match &shuffled_tokens[..] {
                [] => "void".to_string(),
                [first] => format!("'{}' is", first),
                [first, second] => format!("'{}' or '{}' are", first, second),
                [first, second, third] => format!("'{}', '{}', or '{}' are", first, second, third),
                [first, second, third, fourth] => {
                    format!("{}, {}, {}, or {} are", first, second, third, fourth)
                }
                _ => {
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

    fn parse_expression(&mut self) -> Result<Option<Box<dyn ExpressionNode>>> {
        if self.get_current_token().is_err() {
            return Ok(None);
        };
        let current_token = self.get_current_token().unwrap();
        self.move_position();
        match current_token.token_type {
            x if VARIABLE_ASSIGNMENT_TOKENS.contains(&x) => {
                let name_token
                    = self.require_token(vec![TokenType::Alphanumeric]).unwrap();
                self.move_position();
                let datatype = self.parse_datatype().unwrap();
                if datatype.is_none() {
                    self.move_position_back();
                };
                let value_node 
                    = self.parse_assignment()?;
                Ok(Some(
                    Box::new(
                        VariableDeclaration::new(
                        name_token.value,
                        datatype,
                        value_node
                    )))
                )
            }
            x if FORMULA_TOKENS.contains(&x) => {
                self.move_position_back();
                let formula_node = self.require_formula().unwrap();
                Ok(Some(formula_node))
            }
            TokenType::Function => {
                let name_token =
                    self.require_token(vec![TokenType::Alphanumeric]).unwrap();
                self.move_position();
                let arguments
                    = self.parse_parameters_in_parenthesis(ParameterType::Function).unwrap();
                self.move_position();
                let datatype 
                    = self.parse_datatype().unwrap().clone();
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
                Ok(self.parse_expression().unwrap())
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

    fn parse_datatype(&mut self) -> Result<Option<String>> {
        let current_token = self.get_current_token();
        if current_token.is_ok() && current_token.unwrap().is_type(TokenType::Colon) {
            self.move_position();
            let datatype_token = 
                self.require_token(vec![TokenType::Alphanumeric]).unwrap();
            return Ok(Some(datatype_token.value));
        };
        Ok(None)
    } 

    fn parse_assignment(&mut self,) -> Result<Option<Box<dyn ExpressionNode>>> {
        if self.get_current_token().is_ok() 
        && self.move_if_next_token_is(vec![TokenType::Assign]) {
            self.move_position();
            let value_node = 
                self.require_formula()
                .unwrap();
            return Ok(Some(value_node));
        }
        Ok(None)
    }

    fn parse_parameters_in_parenthesis(&mut self, parameter_type: ParameterType) -> Result<Parameters> {
        self.require_token(vec![TokenType::LPar]).unwrap();
        self.move_position();
        let arguments 
            = self.parse_parameters(parameter_type).unwrap();
        let _ = self.move_position();
        self.require_token(vec![TokenType::RPar]).unwrap();
        Ok(arguments)
    } 

    fn parse_parameters(&mut self, parameter_type: ParameterType) -> Result<Parameters> {
        let mut arguments: Parameters = vec![];

        let _is_calling_parameter 
            = parameter_type == ParameterType::Call;

        let mut keyword_arguments_time: bool = false;
        let _check_if_incorrect_argument_sequence
            = |is_keyword_argument: bool, this: &mut Self| {
            if !is_keyword_argument && keyword_arguments_time {
                let current_token = this.get_current_token().unwrap();
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: Positional argument follows keyword argument <-= at {}:{}:{}",
                        "Syntax Error".bright_red(),
                        this.context.code_source,
                        current_token.line + 1,
                        current_token.start + 1
                    )
                ));
            };
            if is_keyword_argument {
                keyword_arguments_time = true;
            };
            Ok(())
        };

        loop {
            if self.get_current_token().is_err()
            {
                self.move_position_back();
                break;      
            };

            let mut first_token = 
                self.get_current_token()?;
            
            if !arguments.is_empty() 
            && first_token.is_type(TokenType::Comma) {
                self.move_position();
                if self.get_current_token().is_err() {
                    return Ok(arguments);
                }
                first_token = 
                    self.get_current_token().unwrap();
            };

            let value_node 
                = self.parse_formula()?;
            if value_node.is_none() {
                self.move_position_back();
                break;
            };
            self.move_position();
            
            let datatype_string 
                = self.parse_datatype()
                .expect("Error occured while datatype parsing");
            if datatype_string.is_none()
            && parameter_type == ParameterType::Function {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: Argument type is expected <-= {}:{}:{}",
                        "Syntax Error".bright_red(),
                        self.context.code_source,
                        self.context.line,
                        self.context.position
                    ),
                ));
            }
            else if datatype_string.is_some()
            && parameter_type == ParameterType::Call {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: Call argument doesn't require type notation <-= {}:{}:{}",
                        "Syntax Error".bright_red(),
                        self.context.code_source,
                        self.context.line,
                        self.context.position
                    )
                ));
            }
            else if datatype_string.is_some() {
                self.move_position();
            };

            arguments.push(Parameter::new(
                first_token.value, 
                datatype_string, 
                Some(value_node.unwrap())
            ));
        }
        Ok(arguments)
    }

    fn parse_identifiers(&mut self) -> Result<Box<dyn ExpressionNode>> {
        let object_token = self.get_current_token().unwrap();
        let mut object_node: Box<dyn ExpressionNode> 
            =  Box::new(IdentifierNode::new(object_token.value));
        if self.move_if_next_token_is(vec![TokenType::LPar]) {
            let arguments
                = self.parse_parameters_in_parenthesis(ParameterType::Call).unwrap();
            object_node = Box::new(
                CallNode::new(object_node, arguments)
            );
        };
        if self.move_if_next_token_is(vec![TokenType::Dot]) { 
            if !self.move_if_position_is_movable() {
                return Err(io::Error::new(
                    io::ErrorKind::Other, 
                    format!(
                        "{}: Children expected <-= at {}:{}:{}",
                        "Syntax Error".bright_red(),
                        self.context.code_source,
                        self.context.line,
                        self.context.position + 1
                    )
                ));
            };
            let next_member 
                = self.parse_identifiers().unwrap();
            object_node = Box::new(
                MemberNode::new(object_node, next_member)
            );
        };
        Ok(object_node)
    }

    fn parse_formula(&mut self) -> Result<Option<Box<dyn ExpressionNode>>> {
        let mut unary_operator_tokens: Vec<Token> = vec![];
        let mut prohibited_unary_operator_types: Vec<TokenType> = vec![];
        
        let is_unary_operator_prohibited
            = move |token_to_check: Token, prohibited_types: Vec<TokenType>, this: &Self| {
            if prohibited_types
                .into_iter()
                .any(|x| token_to_check.is_type(x)) {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: '{}' operator is already used <-= at {}:{}:{}",
                        "Syntax Error".bright_red(),
                        token_to_check.token_type,
                        this.context.code_source,
                        token_to_check.line,
                        token_to_check.start
                    )
                ));
            }  
            Ok(())
        }; 
        
        if self.get_current_token().is_err()
        || !FORMULA_TOKENS.contains(&self.get_current_token()?.token_type)
        {
            return Ok(None);
        };

        loop {
            if UNARY_OPERATOR_TOKENS.contains(&self.get_current_token().unwrap().token_type) {
                let current_unary_operator_token = self.get_current_token().unwrap();
                unary_operator_tokens.push(current_unary_operator_token.clone());
                is_unary_operator_prohibited(
                    current_unary_operator_token.clone(), 
                    prohibited_unary_operator_types.clone(), 
                    self
                ).unwrap();
                self.move_position();
                if [TokenType::Increment, TokenType::Decrement]
                .into_iter()
                .any(|x| x == current_unary_operator_token.token_type) 
                {
                    prohibited_unary_operator_types
                        .extend(vec![TokenType::Increment, TokenType::Decrement]);
                }
                else {
                    prohibited_unary_operator_types
                        .push(current_unary_operator_token.token_type);
                };
                continue
            }
            break;
        };
        let formula_token = self.get_current_token().unwrap();
        let mut left_operand: Box<dyn ExpressionNode>
            = match formula_token.token_type {
            TokenType::Alphanumeric => self.parse_identifiers().unwrap(),
            TokenType::CharArray => Box::new(StringNode::new(formula_token.clone().value)),
            TokenType::Number => Box::new(NumberNode::new(formula_token.value.parse().unwrap())),
            TokenType::Null => Box::new(NullNode),
            TokenType::True | TokenType::False => {
                Box::new(BooleanNode::new(formula_token.token_type.clone()).unwrap())
            }
            _ => {
                self.raise_expected_tokens_error(FORMULA_TOKENS.to_vec()).unwrap();
                Box::new(NullNode{})  
            }
        };
        for unary_operator_token in unary_operator_tokens {
            left_operand = Box::new(
                UnaryOperatorNode::new(
                    unary_operator_token.token_type.clone(), 
                    left_operand,
                    TokenSide::Left
                )
            );
        };
        if self.move_if_next_token_is(BINARY_OPERATOR_TOKENS.to_vec()) {
            let operator = self.get_current_token().unwrap();
            self.move_position();
            let right_operand = self.parse_formula()?.unwrap();
            let binary_operator_node =
                Box::new(BinaryOperatorNode::new(operator.token_type, left_operand, right_operand));
            return Ok(Some(binary_operator_node))
        };
        Ok(Some(left_operand))
    }

    fn require_formula(&mut self) -> Result<Box<dyn ExpressionNode>> {
        let formula_node = self.parse_formula()?;
        if formula_node.is_none() {
            self.raise_expected_tokens_error(FORMULA_TOKENS.to_vec())?;
        }
        Ok(formula_node.unwrap())
    }
}
