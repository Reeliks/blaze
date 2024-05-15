use blaze::db::create_db;
use blaze::scripting::lexer::Lexer;
use blaze::scripting::parser::Parser;
use blaze::scripting::tokens::TokenType;
use blaze::server::headers;

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

fn parser(code: &str) -> std::io::Result<bool> {
    let mut code_lexer = Lexer::new(code.to_string());
    let code_source = String::from("Tests");
    code_lexer.get_context().code_source = code_source.clone();
    let tokens = code_lexer.analyze()?;

    let mut code_parser = Parser::new(tokens.clone());
    code_parser.get_context().code_source = code_source;
    let ast = code_parser.parse();

    Ok(ast.is_ok() && !tokens.is_empty() && !ast?.nodes.is_empty())
}

#[test]
fn test_parser() {
    assert!(parser("fin country_id = 1").unwrap());
    assert!(!parser("fifn country_id = 1").unwrap());
    assert!(parser(
        "function get_best_student_id(schools: list_of_int, min_grade: uint = 1, max_grade: uint = 12) {if empty(schools) {error} else somesearch(min_grade, arg=max_grade)}; result"
    )
    .unwrap());
    assert!(!parser("9 * 12 import").unwrap());
    assert!(parser(
        "mut best_apples: arr = grocery_store.get_best_product_instances(amount=5).result;"
    )
    .unwrap());
}

#[test]
fn test_cteate_db() {
    let is_create = create_db::create_db_structure("./db".trim()).is_ok();
    assert!(is_create);
}

#[test]
fn test_header_parser() {
    let response = "POST / HTTP/1.1\nHost: localhost:3300\nUser-Agent: curl/8.7.1\nAccept: */*\nPassword: 1221\n"
    .to_string();

    let hashmap = headers::parse_header(response).unwrap();
    if let Some(value) = hashmap.get("Password") {
        assert!(value == "1221");
    }
}
