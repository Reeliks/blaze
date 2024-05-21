use crate::routine::formatting::MessagesFormatting;
use crate::routine::info_channel::{get_console_info_channel, InfoChannel};
use crate::scripting::{ast::tokens::Token, lexer, parser};
use crate::server::instance::ServerInstance;
use colored::Colorize;
use std::fs::{self, File};
use std::io::{self, Result, Write};

pub const OFFICIAL_REPOSITORY: &str = "https://github.com/Reeliks/blaze";

pub struct ShellCommandHandler {
    info_channel: InfoChannel,
}

impl Default for ShellCommandHandler {
    fn default() -> Self {
        Self {
            info_channel: get_console_info_channel(),
        }
    }
}

impl ShellCommandHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn handle_command(&mut self) -> Result<()> {
        let mut args: Vec<String> = std::env::args().collect::<Vec<String>>()[1..].to_vec();
        if args.is_empty() {
            args.push("help".to_string());
        };
        let command = args[0].as_str();
        match command {
            "init" => {
                let mut manage_file_name = "main";
                if let Some(first_arg) = args.get(1) {
                    if !first_arg.starts_with('-') {
                        manage_file_name = first_arg
                    }
                }

                if Self::create_management_file(manage_file_name) {
                    println!("Thanks for using {}!", "Blaze Database".yellow());
                    println!("  * To contribute the development process, check out the official repository:\n    {}", OFFICIAL_REPOSITORY);
                } else {
                    println!(
                        "{}: The file/folder with a name '{}' already exists\n{}",
                        "Init Error".bright_red(),
                        manage_file_name,
                        String::from("Try specifying another name").into_hint()
                    );
                }
            }
            "run" | "raise" => self.run_server(args).await?,
            "lexer" => {
                let text = Self::read_console_input()?;
                self.run_lexer(text, true)?;
            }
            "parser" => {
                let text = Self::read_console_input()?;
                self.run_parser(text)?;
            }
            "--help" | "help" => Self::print_help_section(),
            _ => {
                eprintln!("'{}' is not a blaze commmand. See 'blaze --help'", command);
                std::process::exit(1);
            }
        }
        Ok(())
    }

    async fn run_server(&mut self, args: Vec<String>) -> Result<()> {
        if let Err(error) = ServerInstance::new(self.info_channel.clone())
            .launch(&args[1..])
            .await
        {
            eprintln!("{}", error);
        };
        Ok(())
    }

    // Do not indent raw strings below"
    fn print_help_section() {
        let help_list = r#">> Blaze Database 0.0.1a - available commands:
Datablaze Management
    init    - create a new datablaze template to start working blazingly fast
    run     - raise a datablaze configurated in manage.blz | raise
            * manage_file_name, -cons_limit, con_lifetime (secs), -address, -password
        
Blaze Language (Dev)
    lexer   - tokens parsing
    parser  - nodes parsing (lexing included)
"#;

        println!("{}", help_list);
    }

    pub fn create_management_file(name: &str) -> bool {
        let manage_file_content = br#"manage (
    // address = "localhost:3306",
    // sessions_limit = 100,
    // session_lifetime = 60
);

attach "data";"#;

        let manage_file_fullname = &format!(
            "{}.manage.blz",
            if !name.is_empty() { name } else { "main" }
        );
        if fs::metadata(manage_file_fullname).is_err() {
            let mut manage_file = File::create(manage_file_fullname).unwrap();
            manage_file.write_all(manage_file_content).unwrap();
            return true;
        };
        false
    }

    pub fn run_lexer(&self, code_to_parse: String, show_final_message: bool) -> Result<Vec<Token>> {
        let mut code_lexer = lexer::Lexer::new(code_to_parse, self.info_channel.clone());
        code_lexer
            .get_context()
            .set_code_source("Shell".to_string());
        let tokens = code_lexer.analyze()?;

        if !tokens.is_empty() && show_final_message {
            self.info_channel.clone().send(format!(
                "Lexiical Analysis successfully completed! Tokens count: {}",
                tokens.len()
            ))?;
        }
        Ok(tokens)
    }

    pub fn run_parser(&self, code: String) -> Result<()> {
        let tokens = self.run_lexer(code, false)?;
        let mut code_parser = parser::Parser::new(tokens, self.info_channel.clone());
        code_parser
            .get_context()
            .set_code_source("Shell".to_string());
        let nodes = code_parser.parse()?.nodes;
        if !nodes.is_empty() {
            self.info_channel.clone().send(format!(
                "Parsing successfully completed! Nodes Count: {}",
                nodes.len()
            ))?;
        }

        Ok(())
    }

    fn read_console_input() -> io::Result<String> {
        let mut text = String::new();
        std::io::stdin().read_line(&mut text)?;
        Ok(text.trim().to_string())
    }
}
