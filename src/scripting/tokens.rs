use strum_macros::{Display, EnumIter};

#[derive(Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub position: u64,
    pub line: u64,
    pub value: String,
}

impl Token {
    pub fn is_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

// It's necessary to put tokens that structure more longer ones
// below in order to make sure the lexer recognizes tokens properly.
#[derive(Debug, EnumIter, Display, Clone, PartialEq)]
pub enum TokenType {
    // Directives
    Import
    Query,
    // Conditions
    If,
    Else,
    While,
    Continue,
    Break,
    Return,
    // Binary Operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    EqualSign,
    NotEqualSign,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    // Assignment
    Assign,
    Variable,
    Function,
    Enum
    // Brackets
    LPar,
    RPar,
    LBracket,
    RBracket,
    // Types
    CharArray,
    Alphanumeric,
    Number,
    Space,
    Dot,
    Comma,
    Colon,
    True,
    False,
    Null,
    ExpressionEnd,
    // Whitespace
    NewLine,
    Indent,
    Carriage,
}

impl TokenType {
    pub fn regex_str(&self) -> &str {
        match self {
            TokenType::Variable => r"(?:mut|fin)[^\w\d]",
            TokenType::Import => r"import[^\w\d]",
            TokenType::Function => r"function[^\w\d]",
            TokenType::Query => r"(?:new|get|set|del)[^\w\d]",
            TokenType::Enum => r"enum[^\w\d]",
            TokenType::True => r"true[^\w\d]",
            TokenType::False => r"false[^\w\d]",
            TokenType::Null => r"null[^\w\d]",
            TokenType::If => r"if[^\w\d]",
            TokenType::Else => r"else[^\w\d]",
            TokenType::While => r"while[^\w\d]",
            TokenType::Continue => r"continue[^\w\d]",
            TokenType::Break => r"break[^\w\d]",
            TokenType::Return => r"return[^\w\d]",
            TokenType::Addition => r"\+",
            TokenType::Subtraction => r"-",
            TokenType::Multiplication => r"\*",
            TokenType::Division => r"\/",
            TokenType::Assign => r"=",
            TokenType::EqualSign => r"==",
            TokenType::NotEqualSign => r"!=",
            TokenType::Greater => r">",
            TokenType::Less => r"<",
            TokenType::GreaterOrEqual => r">=",
            TokenType::LessOrEqual => r"<=",
            TokenType::LPar => r"\(",
            TokenType::RPar => r"\)",
            TokenType::LBracket => r"\{",
            TokenType::RBracket => r"\}",
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

pub const WHITESPACE_TOKENS: [TokenType; 4] = [
    TokenType::NewLine,
    TokenType::Space,
    TokenType::Indent,
    TokenType::Carriage,
];

pub const BINARY_OPERATOR_TOKENS: [TokenType; 10] = [
    TokenType::Addition,
    TokenType::Subtraction,
    TokenType::Multiplication,
    TokenType::Division,
    TokenType::EqualSign,
    TokenType::NotEqualSign,
    TokenType::Less,
    TokenType::Greater,
    TokenType::LessOrEqual,
    TokenType::GreaterOrEqual,
];

pub const FORMULA_TOKENS: [TokenType; 6] = [
    TokenType::CharArray,
    TokenType::Number,
    TokenType::Alphanumeric,
    TokenType::True,
    TokenType::False,
    TokenType::Null,
];
