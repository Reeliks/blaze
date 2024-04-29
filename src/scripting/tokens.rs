use strum_macros::{Display, EnumIter};

pub struct Token {
    pub token_type: TokenType,
    pub position: u64,
    pub line: u64,
    pub value: String,
}

#[derive(Debug, EnumIter, Display)]
pub enum TokenType {
    VariableAssignment,
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
    Carriage,
}

impl TokenType {
    pub fn regex_str(&self) -> &str {
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
            TokenType::Carriage => r"\r",
        }
    }
}

pub const WHITESPACE_TOKENS: [TokenType; 3] =
    [TokenType::Space, TokenType::Indent, TokenType::Carriage];
