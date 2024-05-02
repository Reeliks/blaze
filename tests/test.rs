use blaze::scripting::lexer::Lexer;
use blaze::scripting::tokens::TokenType;

#[test]
fn test_lexer() {
    let code_to_parse = "if mut while else".to_string();
    let expected_tokens = vec![
        TokenType::If,
        TokenType::Mut,
        TokenType::While,
        TokenType::Else,
    ];

    let code_lexer = Lexer::new(code_to_parse);
    let actual_tokens_result = code_lexer.analyze().unwrap();

    let actual_token_types: Vec<TokenType> = actual_tokens_result
        .iter()
        .map(|token| TokenType::try_from(token.token_type.clone()).unwrap())
        .collect();

    assert_eq!(actual_token_types, expected_tokens);
}
