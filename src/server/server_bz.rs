use crate::{
    server::{config::Config, headers},
    shell::handling::analyze_syntatically,
};
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};

pub fn server_run(args: Vec<String>) -> io::Result<()> {
    let config = Config::parse_arguments(args).unwrap();

    if !Config::blz_exists(&config.manager_file) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "blz_file not found",
        ));
    }

    let host = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(host)?;

    for stream in listener.incoming() {
        let stream = stream?;
        let password = std::mem::take(&mut config.password.clone());

        std::thread::spawn(move || handle_connection(stream, password));
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, password: String) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut request = String::new();
    let bytes_read = stream.read(&mut buffer)?;
    request.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));

    let header = headers::parse_header(request.clone()).unwrap();
    if let Some(value) = header.get("Password") {
        if password == *value {
            analyze_syntatically(headers::remove_empty_line(request).unwrap())?
        };
    };

    Ok(())
}
