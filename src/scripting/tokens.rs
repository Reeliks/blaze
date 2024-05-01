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
    Import,
    Manage,
    Attach,
    Inspect,
    // Conditions
    If,
    Else,
    While,
    Continue,
    Break,
    Return,
    // Unary Operators
    Increment,
    Decrement,
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
    Hat,
    // Assignment
    Assign,
    Mut,
    Fin,
    Function,
    Enum,
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
            TokenType::If => r"if[^\w\d]",
            TokenType::Mut => r"mut[^\w\d]",
            TokenType::Fin => r"fin[^\w\d]",
            TokenType::Enum => r"enum[^\w\d]",
            TokenType::True => r"true[^\w\d]",
            TokenType::False => r"false[^\w\d]",
            TokenType::Null => r"null[^\w\d]",
            TokenType::Else => r"else[^\w\d]",
            TokenType::While => r"while[^\w\d]",
            TokenType::Import => r"import[^\w\d]",
            TokenType::Manage => r"manage[^\w\d]",
            TokenType::Attach => r"attach[^\w\d]",
            TokenType::Inspect => r"inspect[^\w\d]",
            TokenType::Function => r"function[^\w\d]",
            TokenType::Continue => r"continue[^\w\d]",
            TokenType::Break => r"break[^\w\d]",
            TokenType::Return => r"return[^\w\d]",
            TokenType::Increment => r"++",
            TokenType::Decrement => r"--",
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
            TokenType::Hat => r"^",
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

pub const BINARY_OPERATOR_TOKENS: [TokenType; 11] = [
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
    TokenType::Hat,
];

pub const UNARY_OPERATOR_TOKENS: [TokenType; 2] = [TokenType::Increment, TokenType::Decrement];

pub const FORMULA_TOKENS: [TokenType; 6] = [
    TokenType::CharArray,
    TokenType::Number,
    TokenType::Alphanumeric,
    TokenType::True,
    TokenType::False,
    TokenType::Null,
];

pub const VARIABLE_ASSIGNMENT_TOKENS: [TokenType; 2] = [TokenType::Mut, TokenType::Fin];
