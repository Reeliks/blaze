use colored::Colorize;
use ctrlc;

use crate::{
    server::{runtime_config::RuntimeConfig, header_parsing},
    shell::handling::analyze_syntatically,
};
use std::{
    sync::mpsc,
    net::{TcpListener, TcpStream},
    process::exit, 
    io::{self, Result, Read}, 
    path::Path
};

pub fn run_server(args: Vec<String>, info_channel: mpsc::Sender<String>) -> Result<()> {
    let config = RuntimeConfig::parse_arguments(args).unwrap();
    if !RuntimeConfig::blz_exists(&config.manage_file) {
        return Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("{}: {}\n{}\n{}\n{}",
                "Runtime Error".bright_red(),
                "manage.blz not found; couldn't raise a server",
                format_hint_string("Only able to automatically find the management file with a name 'manage.blz'"),
                format_hint_string("Try to specify a path to the management file with '-m <path>' or rename it"),
                format_hint_string("Perhaps the management file doesn't exist?"),
        )));
    }

    info_channel.send(format!(
        "Executing {}",
        Path::new(&config.manage_file) 
        .file_name() 
        .unwrap() 
        .to_string_lossy() 
        .yellow()
    )).unwrap();

    let address = format!("{}:{}", config.host, config.port); 
    let listener = loop { 
        match TcpListener::bind(&address) {
        Ok(success) => {    
            info_channel.send(format!("Gateway opened on {}", address.yellow())).unwrap();
            break success
        },
        Err(error) => {
            if error.kind() == std::io::ErrorKind::AddrInUse {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("{}: {}",
                        "Runtime Error".bright_red(),
                        "The address is already taken; couldn't raise a server"
                )))
            }
            else {
                return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{}: {}",
                        "Runtime Error".bright_red(),
                        error
                ))) 
            }
        }
    }};

    ctrlc::set_handler(move || {
        info_channel.send("Shutting down on ctrl+c...".to_string()).unwrap();
        exit(0);
    }).expect("Error occured while setting ctrl+c handler");

    for stream in listener.incoming() {
        let stream = stream?;
        let password = std::mem::take(&mut config.password.clone());

        std::thread::spawn(move || handle_connection(stream, password));
    }

    Ok(())
}

fn format_hint_string(hint: &str) -> String {
    format!(
        "  * {}",
        hint.bright_blue()
    )
}

fn handle_connection(mut stream: TcpStream, password: String) -> Result<()> {
    let mut buffer = [0; 1024];
    let mut request = String::new();
    let bytes_read = stream.read(&mut buffer)?;
    request.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));

    let header = header_parsing::parse_header(request.clone()).unwrap();
    if let Some(value) = header.get("Password") {
        if password == *value {
            analyze_syntatically(header_parsing::remove_empty_line(request).unwrap())?
        };
    };

    Ok(())
}
