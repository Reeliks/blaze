use crate::server::headers;
use crate::{server::args_parser::Args, shell::handling::analyze_syntatically};
use std::ffi::OsStr;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

pub fn server_run(args: Vec<String>) -> io::Result<()> {
    let mut config = Args::parse(args).unwrap();
    let blz_file = Path::new(&config.blz_file);
    /*
        if !blz_file.exists() || blz_file.extension().unwrap_or(OsStr::new("")) != "blz" {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "blz_file not found",
            ));
        }
    */
    let ip = format!("{}:{}", config.ip, config.port);
    let listener = TcpListener::bind(ip)?;

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

    let hashmap = headers::parse_header(request.clone()).unwrap();
    if let Some(value) = hashmap.get("Password") {
        if password == *value {
            analyze_syntatically(headers::del_headers(request).unwrap())?
        }
    }

    Ok(())
}
