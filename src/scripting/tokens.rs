use strum_macros::{Display, EnumIter};

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start: u64,
    pub stop: u64,
    pub line: u64,
    pub value: String,
}

impl Token {
    pub fn is_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

pub enum TokenSide {
    Left,
    Right,
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
    Elif,
    Else,
    While,
    Continue,
    Break,
    Return,
    // Unary Operators
    Negotion,
    Link,
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
    And,
    Or,
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
    LSquareBracket,
    RSquareBracket,
    // Types
    CharArray,
    Number,
    Space,
    Dot,
    Comma,
    Colon,
    True,
    False,
    Null,
    Alphanumeric,
    // Whitespace
    NewLine,
    Indent,
    Carriage,
    ExpressionEnd,
}

impl TokenType {
    pub fn regex_str(&self) -> &str {
        match self {
            TokenType::If => r"if\b",
            TokenType::Elif => r"elif\b",
            TokenType::Else => r"else\b",
            TokenType::Mut => r"mut\b",
            TokenType::Fin => r"fin\b",
            TokenType::Enum => r"enum\b",
            TokenType::True => r"true\b",
            TokenType::False => r"false\b",
            TokenType::Null => r"null\b",
            TokenType::While => r"while\b",
            TokenType::Import => r"import\b",
            TokenType::Manage => r"manage\b",
            TokenType::Attach => r"attach\b",
            TokenType::Inspect => r"inspect\b",
            TokenType::Function => r"function\b",
            TokenType::Continue => r"continue\b",
            TokenType::Break => r"break\b",
            TokenType::Return => r"return\b",
            TokenType::Negotion => r"\!",
            TokenType::Link => r"&",
            TokenType::Increment => r"\+\+",
            TokenType::Decrement => r"\-\-",
            TokenType::Addition => r"\+",
            TokenType::Subtraction => r"-",
            TokenType::Multiplication => r"\*",
            TokenType::Division => r"\/",
            TokenType::Assign => r"=",
            TokenType::EqualSign => r"==",
            TokenType::NotEqualSign => r"\!=",
            TokenType::Greater => r">",
            TokenType::Less => r"<",
            TokenType::GreaterOrEqual => r">=",
            TokenType::LessOrEqual => r"<=",
            TokenType::And => r"&&",
            TokenType::Or => r"\|\|",
            TokenType::Hat => r"\^",
            TokenType::LPar => r"\(",
            TokenType::RPar => r"\)",
            TokenType::LBracket => r"\{",
            TokenType::RBracket => r"\}",
            TokenType::LSquareBracket => r"\[",
            TokenType::RSquareBracket => r"\]",
            TokenType::CharArray => r#"".*?[^\\]"|"""#,
            TokenType::Alphanumeric => r"[a-zA-Z_]\w*",
            TokenType::Number => r"\d+(\.\d+)?",
            TokenType::Space => r#"\s"#,
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

pub const BINARY_OPERATOR_TOKENS: [TokenType; 14] = [
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
    TokenType::Assign,
    TokenType::And,
    TokenType::Or,
];

pub const UNARY_OPERATOR_TOKENS: [TokenType; 4] = [
    TokenType::Increment,
    TokenType::Decrement,
    TokenType::Negotion,
    TokenType::Link,
];

// The tokens formulas can start with.
pub const FORMULA_TOKENS: [TokenType; 12] = [
    TokenType::CharArray,
    TokenType::Number,
    TokenType::Alphanumeric,
    TokenType::True,
    TokenType::False,
    TokenType::Null,
    TokenType::Increment,
    TokenType::Decrement,
    TokenType::Negotion,
    TokenType::Link,
    TokenType::LBracket,
    TokenType::If,
];

pub const VARIABLE_ASSIGNMENT_TOKENS: [TokenType; 2] = [TokenType::Mut, TokenType::Fin];
