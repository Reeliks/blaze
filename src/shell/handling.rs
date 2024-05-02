use crate::db::create_db;
use crate::scripting::tokens::Token;
use crate::scripting::{lexer, parser};
use crate::server::server_bz;
use std::io::{self, Result};

pub fn handle_command_arguments() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        match args[1].as_str() {
            "create" => create_db_with_console()?,
            "run" => server_bz::server_run(args)?,
            "lexer" => {
                analyze_lexically()?;
            }
            "parser" => {
                analyze_syntatically()?;
            }
            _ => {
                eprintln!("Invalid arguments");
                std::process::exit(1);
            }
        }
    } else {
        print_help_section();
    }
    Ok(())
}

fn print_help_section() {
    let help_list = r#"Blaze Db 0.0.1a - available commands:
    Database management
        create  - create a new datablaze
    Blaze Language
        lexer   - get to see how the code is subjected to lexical analysis under the hood
        parser  - try the first version of a parser
        run     - start server"#;

    println!("{}", help_list);
}

pub fn create_db_with_console() -> Result<()> {
    let mut path = String::new();
    println!("Specify a path to a datablaze");
    io::stdin().read_line(&mut path)?;
    create_db::create_db_structure(path.trim(), true)?;

    Ok(())
}

fn analyze_lexically() -> Result<Vec<Token>> {
    let mut code_to_parse = String::new();
    std::io::stdin().read_line(&mut code_to_parse)?;
    code_to_parse = code_to_parse.trim().to_string();
    let mut code_lexer = lexer::Lexer::new(code_to_parse);
    code_lexer
        .get_context()
        .set_code_source("Shell".to_string());
    Ok(code_lexer.analyze().unwrap())
}

fn analyze_syntatically() -> Result<()> {
    let tokens = analyze_lexically()?;
    let mut code_parser = parser::Parser::new(tokens);
    code_parser
        .get_context()
        .set_code_source("Shell".to_string());
    let nodes = code_parser.parse();
    for _node in &nodes {
        println!("{:#?}", "new node");
    }
    Ok(())
}
