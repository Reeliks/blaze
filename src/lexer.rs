use std::io;
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter}; 

pub struct Token {
    token_type: TokenType,
    position: u32,
    line: u32,
    value: String,
}

#[derive(Debug, EnumIter, Display)]
pub enum TokenType { VariableAssignment,
    FunctionAssignment,
    QueryKeyword,
    ImportKeyword,
    EqualSign,
    Operator,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningCurlyBracket,
    ClosingCurlyBracket,
    CharArray,
    Alphanumeric,
    Number,
    Space,
    Dot,
    Comma,
    Colon,
    ExpressionEnd,
    NewLine,
    Indent,
    Carriage
}

impl TokenType {
    pub fn regex_str (&self) -> &str {
        match self {
            TokenType::VariableAssignment => r"let|var",
            TokenType::QueryKeyword => r"get|set|new|del",
            TokenType::ImportKeyword => r"import",
            TokenType::FunctionAssignment => r"function",
            TokenType::EqualSign => r"=",
            TokenType::Operator => r"[+\-*\/]",
            TokenType::OpeningParenthesis => r"\(",
            TokenType::ClosingParenthesis => r"\)",
            TokenType::OpeningCurlyBracket => r"\{",
            TokenType::ClosingCurlyBracket => r"\}",
            TokenType::CharArray => r#"".*?[^\\]"|"""#,
            TokenType::Alphanumeric => r"[a-zA-Z_]\w*",
            TokenType::Number => r"\d+(\.\d+)?",
            TokenType::Space => " ",
            TokenType::Dot => r"\.",
            TokenType::Comma => r",",
            TokenType::Colon => r":",
            TokenType::ExpressionEnd => r";",
            TokenType::NewLine => r"\n",
            TokenType::Indent => r"\t",
            TokenType::Carriage => r"\r" 
        }
    }
}

const WHITESPACE_TOKENS: [TokenType; 3] = [
    TokenType::Space,
    TokenType::Indent, 
    TokenType::Carriage
];

pub struct CodeContext {
    position: u32,
    line: u32,
    code_source: String
}

impl CodeContext {
    pub fn new (code_source: String) -> Self {
        CodeContext { code_source, line: 0, position: 0 }
    }
}

pub struct Lexer {
    code: String,
    context: CodeContext,
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new (code: String, code_source: String) -> Self {
        Lexer { 
            code,
            context: CodeContext::new(code_source), 
            tokens: vec![] 
        }
    }

    pub fn analyze(mut self) -> io::Result<Vec<Token>>
    {
        loop 
        {
            match self.recognize_next_token() 
            {
                Ok(proceed_parsing) => 
                {
                    if !proceed_parsing
                    {
                        return Ok(self.tokens);
                    }
                    let last_token = self.tokens.last().unwrap();
                    if !WHITESPACE_TOKENS.map(|x| x.to_string()).contains(&last_token.token_type.to_string()) {
                        let start_position = 1 + self.context.position - last_token.value.len() as u32;
                        println!("{}:{} = {}", start_position, last_token.value.to_string(), last_token.token_type.to_string());
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    break;
                }
            };
        };
        Ok(vec![])
    }

    fn recognize_next_token(&mut self) -> io::Result<bool> 
    {
        if self.context.position >= self.code.len().try_into().unwrap()
        {
            return Ok(false);
        };
        let positioned_code = &self.code[self.context.position as usize..];
        for token_type in TokenType::iter()
        {
            let token_regex = Regex::new(format!("^{}",TokenType::regex_str(&token_type)).as_str()).unwrap();
            if let Some(matches) = token_regex.find(positioned_code)
            {
                let matched_string = matches.as_str();
                self.tokens.push(
                    Token {
                        token_type,
                        position: self.context.position,
                        line: self.context.line,
                        value: matched_string.to_string()
                    }
                );
                if token_regex.clone().to_string() == TokenType::ExpressionEnd.to_string() {
                    self.context.line += 1;
                };
                self.context.position += matched_string.len() as u32;
                self.find_lexical_errors()?;
                return Ok(true)
            }
        }
        Err(
            io::Error::new(
                io::ErrorKind::Other,
                format!("\n\"{}\" token isn't registered <-= at {}:{}:{}", 
                    positioned_code, 
                    self.context.code_source,
                    self.context.line + 1,
                    self.context.position + 1
                ).to_string()
            )
        ) 
    }

    fn find_lexical_errors (&mut self) -> io::Result<()> {
        self.throw_error_if_number_and_string_together()?;
        self.throw_error_if_unresolved_chars_near_string()?;
        Ok(())
    }

    fn throw_error_if_number_and_string_together (&self) -> io::Result<()> {
        if self.tokens.len() >= 2 {
            let current_token = self.tokens.last().unwrap();
            let last_token = self.tokens.get(self.tokens.len() - 2).unwrap();

            let current_token_is_alphanumeric = current_token.token_type.to_string() == TokenType::Alphanumeric.to_string();
            let last_token_is_number = last_token.token_type.to_string() == TokenType::Number.to_string();
            if last_token_is_number && current_token_is_alphanumeric
            {
                return Err(
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "\n\"{}{}\": numbers cannot end with alphanumeric <-= at {}:{}:{}", 
                            last_token.value, 
                            current_token.value,
                            self.context.code_source,
                            last_token.line + 1,
                            last_token.position + 1
                        ).to_string()
                    )
                );
            }
        }
        Ok(())
    }

    fn throw_error_if_unresolved_chars_near_string (&mut self) -> io::Result<()> {
        let current_token = self.tokens.last().unwrap();
        if self.tokens.len() >= 2 && current_token.token_type.to_string() == TokenType::CharArray.to_string() {
            let both_sides_unresolved_chars_regex = Regex::new(r"[\w\d]").unwrap();
            let left_side_unresolved_chars_regex = Regex::new(r"[\.]").unwrap();

            let char_before = &self.code.chars().nth((current_token.position - 1) as usize).unwrap().to_string();
            let char_after = &self.code.chars().nth(current_token.position as usize + current_token.value.len()).unwrap().to_string();
            println!("{}", char_after);

            let error_address = format!("{}:{}:{}", self.context.code_source, current_token.line + 1, current_token.position + 1);
            if both_sides_unresolved_chars_regex.is_match(char_before) || left_side_unresolved_chars_regex.is_match(&char_before) 
            {
                return Err(
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "\n\"{}\" near a string with no space between <-= at {}",
                            char_before, error_address
                        )
                    )
                );
            }
            else if both_sides_unresolved_chars_regex.is_match(char_after)
            {
                return Err(
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "\n\"{}\" after a string with no space between <-= at {}",
                            char_after, error_address
                        )
                    )
                )
            }
        } 

        Ok(())
    }
    
}
