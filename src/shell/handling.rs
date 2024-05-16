use colored::Colorize;

use crate::scripting::tokens::Token;
use crate::scripting::{lexer, parser};
use crate::server::management;
use std::fs::{self, File};
use std::io::Write;
use std::sync::mpsc;
use std::{
    io::{self, Result},
    thread
};

pub const OFFICIAL_REPOSITORY: &str = "https://github.com/Reeliks/blaze";

pub fn handle_command() -> Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        args.push("help".to_string());
    };
    let command = args[1].as_str();
    match command {
        "init" => {
            if create_management_file() {
                println!("Thanks for using {}!", "Blaze Database".yellow());
                println!("  * To contribute the development process, check out the official repository:\n    {}", OFFICIAL_REPOSITORY);
            }
            else {
                println!("{}: The file/folder with a name 'manage.blz' already exists, couldn't initialize", "Init Error".bright_red());
            }
        },
        "run" | "raise" => run_server_with_console_output(args)?,
        "lexer" => {
            let text = input_text()?;
            analyze_lexically(text)?;
        }
        "parser" => {
            let text = input_text()?;
            analyze_syntatically(text)?;
        }
        "--help" | "help" => {
            print_help_section()
        }
        _ => {
            eprintln!("'{}' is not a blaze commmand. See 'blaze --help'", command);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn run_server_with_console_output(args: Vec<String>) -> Result<()> {
    let (info_channel_tx, info_channel_rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        loop {
            if let Ok(received) = info_channel_rx.try_recv() {
                println!("{}", received);
            }         
        }
    });
    
    if let Err(error) = management::run_server(args, info_channel_tx) {
        eprintln!("{}", error);
    };
    Ok(())
}

fn print_help_section() {
    let help_list = r#">> Blaze Database 0.0.1a - available commands:
Datablaze Management
    init    - create a new datablaze template to start working blazingly fast | create
    run     - raise a datablaze configurated in manage.blz | raise
    
Blaze Language (Dev)
    lexer   - tokens parsing
    parser  - nodes parsing (lexing included)
"#;

    println!("{}", help_list);
}

pub fn create_management_file() -> bool {
    let manage_file_content = br#"manage (
    // port = "3305",
    // host = "127.0.0.1",
    // connections_limit = 10,
);

attach "data";"#;
    if fs::metadata("manage.blz").is_err() {
        let mut managing_file = File::create(&mut "manage.blz").unwrap();
        managing_file.write_all(manage_file_content).unwrap();
        return true;
    };
    false
    
}

fn analyze_lexically(code_to_parse: String) -> Result<Vec<Token>> {
    let mut code_lexer = lexer::Lexer::new(code_to_parse);
    code_lexer
        .get_context()
        .set_code_source("Shell".to_string());
    Ok(code_lexer.analyze().unwrap())
}

pub fn analyze_syntatically(code: String) -> Result<()> {
    let tokens = analyze_lexically(code)?;
    let mut code_parser = parser::Parser::new(tokens);
    code_parser
        .get_context()
        .set_code_source("Shell".to_string());
    let nodes = code_parser.parse()?.nodes;
    if !nodes.is_empty() {
        println!(
            "Parsing successfully completed! Nodes Count: {}",
            nodes.len()
        );
    }
    Ok(())
}

fn input_text() -> io::Result<String> {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text)?;
    Ok(text)
}
