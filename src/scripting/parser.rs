use super::ast::binary_operator::BinaryOperatorNode;
use super::ast::body::BodyNode;
use super::ast::boolean::BooleanNode;
use super::ast::call::CallNode;
use super::ast::conditional_tree::{ConditionalTreeNode, Conditions};
use super::ast::expression::ExpressionNode;
use super::ast::function_declaration::FunctionDeclarationNode;
use super::ast::functional_return::FunctionalReturnNode;
use super::ast::identifier::IdentifierNode;
use super::ast::loop_control::{LoopControlNode, LoopControlType};
use super::ast::member::MemberNode;
use super::ast::null::NullNode;
use super::ast::number::NumberNode;
use super::ast::parameter::{Parameter, ParameterType, Parameters};
use super::ast::string::StringNode;
use super::ast::unary_operator::UnaryOperatorNode;
use super::ast::variable_declaration::VariableDeclaration;
use super::ast::while_loop::WhileLoopNode;
use super::context::Context;
use super::tokens::{
    Token, TokenSide, TokenType, BINARY_OPERATOR_TOKENS, FORMULA_TOKENS, UNARY_OPERATOR_TOKENS,
    VARIABLE_ASSIGNMENT_TOKENS,
};
use colored::*;
use rand::seq::SliceRandom;
use std::io::{self, Result};

pub struct Parser {
    tokens: Vec<Token>,
    context: Context,
    parser_position: u64,

    syntax_error_marking: ColoredString,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            context: Context::default(),
            tokens,
            parser_position: 0,

            syntax_error_marking: "Syntax Error".bright_red(),
        }
    }

    pub fn parse(&mut self) -> Result<BodyNode> {
        let body = self.run_parsing_logic();
        if let Err(error) = body {
            eprintln!("{}", error);
            return Ok(BodyNode::new());
        }
        body
    }

    fn run_parsing_logic(&mut self) -> Result<BodyNode> {
        let body = self.parse_body_outside_brackets()?;
        if self.get_current_token().is_ok()
            && self.get_current_token()?.is_type(TokenType::RBracket)
        {
            self.raise_unexpected_tokens_error(vec![])?;
        };
        Ok(body)
    }

    fn parse_body(&mut self) -> Result<Option<BodyNode>> {
        if self.get_current_token().is_err()
            || !self.get_current_token()?.is_type(TokenType::LBracket)
        {
            return Ok(None);
        }
        self.move_position();
        let body = self.parse_body_outside_brackets()?;
        self.require_token(vec![TokenType::RBracket])?;
        Ok(Some(body))
    }

    fn require_body(&mut self) -> Result<BodyNode> {
        let body = self.parse_body()?;
        if body.is_none() {
            self.raise_unexpected_tokens_error(vec![TokenType::LBracket])?;
        }
        Ok(body.unwrap())
    }

    fn parse_body_outside_brackets(&mut self) -> Result<BodyNode> {
        let mut root = BodyNode::new();
        let mut add_node = |node: Box<dyn ExpressionNode>| {
            root.add_node(node);
        };
        loop {
            if self.get_current_token().is_err() {
                break;
            };

            match self.get_current_token()?.token_type {
                TokenType::RBracket => {
                    break;
                }
                TokenType::ExpressionEnd => {
                    self.move_position();
                    continue;
                }
                _ => {}
            };

            let parsed_expression = self.parse_expression()?;
            add_node(parsed_expression);
            self.move_position();

            if let Ok(next_token) = self.get_current_token() {
                if !next_token.is_type(TokenType::ExpressionEnd)
                    && !next_token.is_type(TokenType::RBracket)
                {
                    self.raise_unexpected_tokens_error(vec![TokenType::ExpressionEnd])?;
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
            .any(|x| x == self.get_current_token().unwrap().token_type)
        {
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
        self.raise_unexpected_tokens_error(expected_tokens)?;
        current_token
    }

    fn raise_unexpected_tokens_error(&mut self, expected_tokens: Vec<TokenType>) -> Result<()> {
        let mut shuffled_tokens = expected_tokens;
        let current_token = self.get_current_token();
        let error_location_notation = format!(
            "<-= {}:{}:{}",
            self.context.code_source, self.context.line, self.context.position
        );
        shuffled_tokens.shuffle(&mut rand::thread_rng());
        let error_message = if !shuffled_tokens.is_empty() {
            format!(
                "{}: expected {}{} {}",
                self.syntax_error_marking,
                &match &shuffled_tokens[..] {
                    [] => "void".to_string(),
                    [first] => format!("'{}'", first),
                    [first, second] => format!("'{}' or '{}'", first, second),
                    [first, second, third] => format!("'{}', '{}', or '{}'", first, second, third),
                    [first, second, third, fourth] => {
                        format!("{}, {}, {}, or {}", first, second, third, fourth)
                    }
                    _ => {
                        format!(
                            "'{}', '{}', '{}', or one of {} other tokens",
                            shuffled_tokens[0],
                            shuffled_tokens[1],
                            shuffled_tokens[2],
                            shuffled_tokens.len() - 3
                        )
                    }
                },
                if current_token.is_ok() {
                    format!(", found '{}'", current_token?.value)
                } else {
                    String::new()
                },
                error_location_notation
            )
        } else {
            format!(
                "{}: unexpected '{}' found {}",
                self.syntax_error_marking, current_token?.value, error_location_notation
            )
        };
        Err(io::Error::new(io::ErrorKind::Other, error_message))
    }

    fn parse_expression(&mut self) -> Result<Box<dyn ExpressionNode>> {
        let current_token = self.get_current_token()?;
        match current_token.token_type {
            x if VARIABLE_ASSIGNMENT_TOKENS.contains(&x) => {
                self.move_position();
                let name_token = self.require_token(vec![TokenType::Alphanumeric])?;
                let datatype = self.parse_datatype()?;
                let value_node = self.parse_assignment()?;
                Ok(Box::new(VariableDeclaration::new(
                    name_token.value,
                    datatype,
                    value_node,
                )))
            }
            x if FORMULA_TOKENS.contains(&x) => {
                let formula_node = self.require_formula()?;
                Ok(formula_node)
            }
            TokenType::Function => {
                self.move_position();
                let name_token = self.require_token(vec![TokenType::Alphanumeric])?;
                self.move_position();
                let arguments = self.parse_parameters_in_parenthesis(ParameterType::Function)?;
                let datatype = self.parse_datatype()?;
                self.move_position();
                Ok(Box::new(FunctionDeclarationNode::new(
                    name_token.value,
                    datatype,
                    arguments,
                    Some(self.require_body()?),
                )))
            }
            TokenType::Return => {
                let returned_formula_node = if self.move_if_next_token_is(FORMULA_TOKENS.to_vec()) {
                    Some(self.require_formula()?)
                } else {
                    None
                };
                Ok(Box::new(FunctionalReturnNode::new(returned_formula_node)))
            }
            TokenType::While => {
                self.move_position();
                let condition_node = self.require_formula()?;
                self.move_position();
                Ok(Box::new(WhileLoopNode::new(
                    condition_node,
                    self.require_body()?,
                )))
            }
            TokenType::Continue => Ok(Box::new(LoopControlNode::new(LoopControlType::Continue))),
            TokenType::Break => Ok(Box::new(LoopControlNode::new(LoopControlType::Break))),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "{}: {} is not being considered in this context <-= at {}:{}:{}",
                    self.syntax_error_marking,
                    current_token.token_type,
                    self.context.code_source,
                    self.context.line,
                    self.context.position
                ),
            )),
        }
    }

    fn parse_datatype(&mut self) -> Result<Option<String>> {
        if self.move_if_next_token_is(vec![TokenType::Colon]) {
            self.move_position();
            let datatype_token = self.require_token(vec![TokenType::Alphanumeric])?;
            return Ok(Some(datatype_token.value));
        };
        Ok(None)
    }

    fn parse_assignment(&mut self) -> Result<Option<Box<dyn ExpressionNode>>> {
        if self.move_if_next_token_is(vec![TokenType::Assign]) {
            self.move_position();
            let value_node = self.require_formula()?;
            return Ok(Some(value_node));
        }
        Ok(None)
    }

    fn parse_parameters_in_parenthesis(
        &mut self,
        parameter_type: ParameterType,
    ) -> Result<Parameters> {
        self.require_token(vec![TokenType::LPar])?;
        self.move_position();
        let arguments = self.parse_parameters(parameter_type)?;
        let _ = self.move_position();
        self.require_token(vec![TokenType::RPar])?;
        Ok(arguments)
    }

    fn parse_parameters(&mut self, parameter_type: ParameterType) -> Result<Parameters> {
        let mut arguments: Parameters = vec![];

        let mut keyword_arguments_time: bool = false;
        let mut check_if_incorrect_argument_sequence =
            |is_keyword_argument: bool, this: &mut Self| {
                if !is_keyword_argument && keyword_arguments_time {
                    let current_token = this.get_current_token()?;
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "{}: Positional argument follows keyword argument <-= at {}:{}:{}",
                            this.syntax_error_marking,
                            this.context.code_source,
                            current_token.line + 1,
                            current_token.start + 1
                        ),
                    ));
                };
                if is_keyword_argument {
                    keyword_arguments_time = true;
                };
                Ok(())
            };

        loop {
            if self.get_current_token().is_err()
                || !FORMULA_TOKENS.contains(&self.get_current_token()?.token_type)
            {
                self.move_position_back();
                break;
            };

            let first_token = self.get_current_token()?;

            match parameter_type {
                ParameterType::Function => {
                    if !first_token.is_type(TokenType::Alphanumeric) {
                        self.move_position_back();
                        break;
                    };
                    let datatype_string = self
                        .parse_datatype()
                        .expect("Error occured while datatype parsing");
                    if datatype_string.is_none() {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "{}: Argument type is expected <-= {}:{}:{}",
                                self.syntax_error_marking,
                                self.context.code_source,
                                self.context.line,
                                self.context.position
                            ),
                        ));
                    }
                    let default_value = self.parse_assignment()?;
                    check_if_incorrect_argument_sequence(default_value.is_some(), self)?;
                    arguments.push(Parameter::new(
                        first_token.value,
                        datatype_string,
                        default_value,
                    ));
                }
                ParameterType::Call => {
                    let assignment = self.parse_assignment()?;
                    let is_assignment = assignment.is_some();
                    let argument_value =
                        if first_token.is_type(TokenType::Alphanumeric) && is_assignment {
                            check_if_incorrect_argument_sequence(true, self)?;
                            assignment
                        } else {
                            check_if_incorrect_argument_sequence(false, self)?;
                            self.parse_formula()?
                        };
                    arguments.push(Parameter::new(
                        if is_assignment {
                            first_token.value
                        } else {
                            String::new()
                        },
                        None,
                        argument_value,
                    ));
                }
            };
            self.move_position();
            if self.get_current_token().is_ok()
                && self.get_current_token()?.is_type(TokenType::Comma)
            {
                self.move_position();
            }
        }
        Ok(arguments)
    }

    fn parse_identifiers(&mut self) -> Result<Box<dyn ExpressionNode>> {
        let object_token = self.get_current_token()?;
        let mut object_node: Box<dyn ExpressionNode> =
            Box::new(IdentifierNode::new(object_token.value));
        if self.move_if_next_token_is(vec![TokenType::LPar]) {
            let arguments = self.parse_parameters_in_parenthesis(ParameterType::Call)?;
            object_node = Box::new(CallNode::new(object_node, arguments));
        };
        if self.move_if_next_token_is(vec![TokenType::Dot]) {
            if !self.move_if_position_is_movable() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "{}: Child call expected <-= at {}:{}:{}",
                        self.syntax_error_marking,
                        self.context.code_source,
                        self.context.line,
                        self.context.position + 1
                    ),
                ));
            };
            let next_member = self.parse_identifiers()?;
            object_node = Box::new(MemberNode::new(object_node, next_member));
        };
        Ok(object_node)
    }


    fn parse_formula(&mut self) -> Result<Option<Box<dyn ExpressionNode>>> {
        let mut unary_operator_tokens: Vec<Token> = vec![];
        let mut prohibited_unary_operator_types: Vec<TokenType> = vec![];

        let is_unary_operator_prohibited =
            move |token_to_check: Token, prohibited_types: Vec<TokenType>, this: &Self| {
                if prohibited_types
                    .into_iter()
                    .any(|x| token_to_check.is_type(x))
                {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "{}: '{}' operator is already used <-= at {}:{}:{}",
                            this.syntax_error_marking,
                            token_to_check.token_type,
                            this.context.code_source,
                            token_to_check.line,
                            token_to_check.start
                        ),
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
            if UNARY_OPERATOR_TOKENS.contains(&self.get_current_token()?.token_type) {
                let current_unary_operator_token = self.get_current_token()?;
                unary_operator_tokens.push(current_unary_operator_token.clone());
                is_unary_operator_prohibited(
                    current_unary_operator_token.clone(),
                    prohibited_unary_operator_types.clone(),
                    self,
                )?;
                self.move_position();
                if [TokenType::Increment, TokenType::Decrement]
                    .into_iter()
                    .any(|x| x == current_unary_operator_token.token_type)
                {
                    prohibited_unary_operator_types
                        .extend(vec![TokenType::Increment, TokenType::Decrement]);
                    continue;
                }
                prohibited_unary_operator_types.push(current_unary_operator_token.token_type);
                continue;
            }
            break;
        }
        let formula_token = self.get_current_token()?;
        let mut left_operand: Box<dyn ExpressionNode> = match formula_token.token_type {
            TokenType::Alphanumeric => self.parse_identifiers()?,
            TokenType::CharArray => Box::new(StringNode::new(formula_token.value)),
            TokenType::Number => Box::new(NumberNode::new(formula_token.value.parse().unwrap())),
            TokenType::Null => Box::new(NullNode),
            TokenType::True | TokenType::False => {
                Box::new(BooleanNode::new(formula_token.token_type)?)
            }
            TokenType::If => {
                let mut conditions: Conditions = vec![];

                self.move_position();
                let condition_node = self.require_formula()?;
                self.move_position();
                conditions.push((condition_node, self.require_formula()?));

                loop {
                    if self.move_if_next_token_is(vec![TokenType::Elif]) {
                        self.move_position();
                        let conditional_node = self.require_formula()?;
                        self.move_position();
                        conditions.push((conditional_node, self.require_formula()?));
                        continue;
                    }
                    break;
                }

                let default_node = if self.move_if_next_token_is(vec![TokenType::Else]) {
                    self.move_position();
                    Some(self.require_formula()?)
                } else {
                    None
                };

                Box::new(ConditionalTreeNode::new(conditions, default_node))
            }
            TokenType::LBracket => {
                let body_node = self.require_body()?;
                Box::new(body_node)
            }
            _ => {
                self.raise_unexpected_tokens_error(FORMULA_TOKENS.to_vec())?;
                Box::new(NullNode {})
            }
        };
        for unary_operator_token in unary_operator_tokens {
            left_operand = Box::new(UnaryOperatorNode::new(
                unary_operator_token.token_type,
                left_operand,
                TokenSide::Left,
            ));
        }
        if self.move_if_next_token_is(BINARY_OPERATOR_TOKENS.to_vec()) {
            let operator = self.get_current_token()?;
            self.move_position();
            let right_operand = self.require_formula()?;
            let binary_operator_node = Box::new(BinaryOperatorNode::new(
                operator.token_type,
                left_operand,
                right_operand,
            ));
            return Ok(Some(binary_operator_node));
        };
        Ok(Some(left_operand))
    }

    fn require_formula(&mut self) -> Result<Box<dyn ExpressionNode>> {
        let formula_node = self.parse_formula()?;
        if formula_node.is_none() {
            self.raise_unexpected_tokens_error(FORMULA_TOKENS.to_vec())?;
        }
        Ok(formula_node.unwrap())
    }
}
