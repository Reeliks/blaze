use crate::{server::args_parser::Args, shell::handling::analyze_syntatically};
use std::ffi::OsStr;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

pub fn server_run(args: Vec<String>) -> io::Result<()> {
    let config = Args::parse(args).unwrap();
    let blz_file = Path::new(&config.blz_file);

    if !blz_file.exists() || blz_file.extension().unwrap_or(OsStr::new("")) != "blz" {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "blz_file not found",
        ));
    }

    let ip = format!("{}:{}", config.ip, config.port);
    let listener = TcpListener::bind(ip)?;

    for stream in listener.incoming() {
        let stream = stream?;
        std::thread::spawn(move || handle_connection(stream));
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    let bytes_read = stream.read(&mut buffer)?;

    request.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));

    let test: Vec<&str> = request.split("\n").collect();

    analyze_syntatically(test.last().unwrap().to_string())?;

    Ok(())
}