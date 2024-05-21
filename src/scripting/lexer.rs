use colored::Colorize;
use regex::Regex;
use std::io;
use strum::IntoEnumIterator;

use crate::routine::info_channel::InfoChannel;

use super::ast::tokens::{Token, TokenType, WHITESPACE_TOKENS};
use super::context::Context;
use crate::prelude::*;

pub struct Lexer {
    pub context: Context,
    code: String,
    tokens: Vec<Token>,
    info_channel: InfoChannel, // Connect it only on debugging
}

impl Lexer {
    pub fn new(code: String, info_channel: InfoChannel) -> Self {
        Self {
            code,
            context: Context::default(),
            tokens: vec![],
            info_channel,
        }
    }

    pub fn get_context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn analyze(mut self) -> io::Result<Vec<Token>> {
        loop {
            match self.recognize_next_token() {
                Ok(proceed_parsing) => {
                    if !proceed_parsing {
                        self.tokens.retain(|token| !token.is_type(TokenType::Space));
                        return Ok(self.tokens.clone());
                    }
                    let last_token = self.tokens.last().unwrap();
                    if !WHITESPACE_TOKENS.contains(&last_token.token_type) {
                        let start_position =
                            1 + self.context.position - last_token.value.len() as u64;
                        self.info_channel
                            .clone()
                            .send(format!(
                                "{}:{} = {}",
                                start_position, last_token.value, last_token.token_type
                            ))
                            .unwrap();
                    }
                }
                Err(err) => {
                    self.info_channel.send(err.to_string()).unwrap();
                    break;
                }
            };
        }
        Ok(vec![])
    }

    fn recognize_next_token(&mut self) -> io::Result<bool> {
        if self.context.position >= self.code.len().try_into().unwrap() {
            return Ok(false);
        };
        let positioned_code = &self.code[self.context.position as usize..];
        for token_type in TokenType::iter() {
            let token_regex_string = TokenType::regex_str(&token_type);
            let token_regex = Regex::new(&format!(r#"^{}"#, token_regex_string)).unwrap();
            if let Some(matches) = token_regex.find(positioned_code) {
                let matched_str = if token_type == TokenType::Number {
                    matches.as_str().replace('_', "")
                } else {
                    matches.as_str().to_string()
                };
                self.tokens.push(Token {
                    token_type,
                    start: self.context.position,
                    stop: self.context.position + matched_str.len() as u64,
                    line: self.context.line,
                    value: matched_str.to_string(),
                });
                if matched_str == TokenType::ExpressionEnd.to_string() {
                    self.context.line += 1;
                };
                self.context.position += matched_str.len() as u64;
                self.find_lexical_errors()?;
                return Ok(true);
            }
        }
        Err(io::Error::new(
            ErrorKind::Other,
            format!(
                "{}: '{}' {} <-= at {}:{}:{}",
                "Lexical Error".bright_red(),
                positioned_code.trim(),
                "is not recognized",
                self.context.code_source,
                self.context.line + 1,
                self.context.position + 1
            ),
        ))
    }

    fn find_lexical_errors(&mut self) -> io::Result<()> {
        self.throw_error_if_alphanumeric_in_number()?;
        self.throw_error_if_unresolved_chars_near_string()?;
        Ok(())
    }

    fn throw_error_if_alphanumeric_in_number(&self) -> io::Result<()> {
        if self.tokens.len() >= 2 {
            let current_token = self.tokens.last().unwrap();
            let last_token = self.tokens.get(self.tokens.len() - 2).unwrap();

            let current_token_is_alphanumeric = current_token.is_type(TokenType::Alphanumeric);
            let last_token_is_number = last_token.is_type(TokenType::Number);
            if last_token_is_number && current_token_is_alphanumeric {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    format!(
                        "{}: numbers cannot end with alphanumeric <-= at {}:{}:{}",
                        "Lexical Error".bright_red(),
                        self.context.code_source,
                        last_token.line + 1,
                        last_token.start + 1
                    )
                    .to_string(),
                ));
            }
        }
        Ok(())
    }

    fn throw_error_if_unresolved_chars_near_string(&mut self) -> io::Result<()> {
        let current_token = self.tokens.last().unwrap();

        if !self.tokens.is_empty() && current_token.is_type(TokenType::CharArray) {
            let both_sides_unresolved_chars_regex = Regex::new(r"[\w\d]").unwrap();
            let left_side_unresolved_chars_regex = Regex::new(r"[\.]").unwrap();

            let char_before_index: u64 = current_token.start - 1;
            let char_after_index: u64 = current_token.start + current_token.value.len() as u64;

            if char_before_index as i32 > 0 {
                let char_before = &self
                    .code
                    .chars()
                    .nth((current_token.start - 1) as usize)
                    .unwrap()
                    .to_string();
                if both_sides_unresolved_chars_regex.is_match(char_before)
                    || left_side_unresolved_chars_regex.is_match(char_before)
                {
                    return Err(io::Error::new(
                        ErrorKind::Other,
                        format!(
                            "{}: \"{}\" before a string with no space between <-= at {}:{}:{}",
                            "Lexical Error".bright_red(),
                            char_before,
                            self.context.code_source,
                            current_token.line + 1,
                            char_before_index + 1
                        ),
                    ));
                }
            };
            if char_after_index < self.code.len() as u64 {
                let char_after = &self
                    .code
                    .chars()
                    .nth(char_after_index as usize)
                    .unwrap()
                    .to_string();
                if both_sides_unresolved_chars_regex.is_match(char_after) {
                    return Err(io::Error::new(
                        ErrorKind::Other,
                        format!(
                            "{}: \"{}\" after a string with no space between <-= at {}:{}:{}",
                            "Lexical Error".bright_red(),
                            char_after,
                            self.context.code_source,
                            current_token.line + 1,
                            char_after_index + 1
                        ),
                    ));
                }
            };
        }

        Ok(())
    }
}
