use blaze::db::create_db;
use blaze::scripting::lexer::Lexer;
use blaze::scripting::parser::Parser;
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

fn parser(code: String) -> std::io::Result<bool> {
    let code_lexer = Lexer::new(code);
    let actual_tokens_result = code_lexer.analyze()?;

    let mut code_parser = Parser::new(actual_tokens_result);
    let nodes = code_parser.parse();

    Ok(nodes.is_ok())
}

#[test]
fn test_parser() {
    assert!(parser("fin country_id = 1".to_string()).unwrap());
    assert!(parser("fin country_id = 1".to_string()).unwrap());
}

#[test]
fn test_cteate_db() {
    let is_create = create_db::create_db_structure("./db".trim(), true).is_ok();
    assert!(is_create);
}
