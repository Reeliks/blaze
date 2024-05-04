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
    let tokens = code_lexer.analyze().unwrap();

    let actual_token_types: Vec<TokenType> = tokens
        .iter()
        .map(|token| TokenType::try_from(token.token_type.clone()).unwrap())
        .collect();

    assert_eq!(actual_token_types, expected_tokens);
}

fn parser(code: String) -> std::io::Result<bool> {
    let code_lexer = Lexer::new(code);
    let tokens = code_lexer.analyze()?;

    let mut code_parser = Parser::new(tokens.clone());
    let ast = code_parser.parse();

    Ok(ast.is_ok() && !tokens.is_empty() && !ast?.nodes.is_empty())
}

#[test]
fn test_parser() {
    assert!(parser("fin country_id = 1".to_string()).unwrap());
    assert!(!parser("fifn country_id = 1".to_string()).unwrap());
    assert!(parser(
        "function get_cheapest_cure(disease_name: str, pharmacy_is_open: bool): link;".to_string()
    )
    .unwrap());
    assert!(!parser("9 * 12 import".to_string()).unwrap());
}

#[test]
fn test_cteate_db() {
    let is_create = create_db::create_db_structure("./db".trim(), true).is_ok();
    assert!(is_create);
}
