use regex::Regex;
use std::io;
use strum::IntoEnumIterator;

use super::context::Context;
use super::tokens::{Token, TokenType, WHITESPACE_TOKENS};

pub struct Lexer {
    pub context: Context,
    code: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer {
            code,
            context: Context::default(),
            tokens: vec![],
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
                        return Ok(self.tokens);
                    }
                    let last_token = self.tokens.last().unwrap();
                    if !WHITESPACE_TOKENS.contains(&last_token.token_type)
                    {
                        let start_position =
                            1 + self.context.position - last_token.value.len() as u64;
                        println!(
                            "{}:{} = {}",
                            start_position, last_token.value, last_token.token_type
                        );
                    }
                }
                Err(err) => {
                    eprintln!("{}", err);
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
            let token_regex =
                Regex::new(format!("^{}", TokenType::regex_str(&token_type)).as_str()).unwrap();
            if let Some(matches) = token_regex.find(positioned_code) {
                let matched_str = matches.as_str();
                self.tokens.push(Token {
                    token_type,
                    position: self.context.position,
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
            io::ErrorKind::Other,
            format!(
                "\n\"{}\" token isn't recognized <-= at {}:{}:{}",
                positioned_code,
                self.context.code_source,
                self.context.line + 1,
                self.context.position + 1
            )
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

            let current_token_is_alphanumeric =
                current_token.token_type == TokenType::Alphanumeric;
            let last_token_is_number =
                last_token.token_type == TokenType::Number;
            if last_token_is_number && current_token_is_alphanumeric {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "\n\"{}{}\": numbers cannot end with alphanumeric <-= at {}:{}:{}",
                        last_token.value,
                        current_token.value,
                        self.context.code_source,
                        last_token.line + 1,
                        last_token.position + 1
                    )
                    .to_string(),
                ));
            }
        }
        Ok(())
    }

    fn throw_error_if_unresolved_chars_near_string(&mut self) -> io::Result<()> {
        let current_token = self.tokens.last().unwrap();

        if !self.tokens.is_empty() && current_token.token_type == TokenType::CharArray {

            let both_sides_unresolved_chars_regex = Regex::new(r"[\w\d]").unwrap();
            let left_side_unresolved_chars_regex = Regex::new(r"[\.]").unwrap();

            let char_before_index: u64 = current_token.position - 1;
            let char_after_index: u64 = current_token.position + current_token.value.len() as u64;

            if char_before_index as i32 > 0 {
                let char_before = &self
                    .code
                    .chars()
                    .nth((current_token.position - 1) as usize)
                    .unwrap()
                    .to_string();
                if both_sides_unresolved_chars_regex.is_match(char_before)
                    || left_side_unresolved_chars_regex.is_match(char_before)
                {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "\n\"{}\" near a string with no space between <-= at {}:{}:{}",
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
                        io::ErrorKind::Other,
                        format!(
                            "\n\"{}\" after a string with no space between <-= at {}:{}:{}",
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
