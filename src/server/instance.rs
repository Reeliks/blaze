use chrono::Local;
use colored::Colorize;
use ctrlc;

use super::{client_connection::ClientConnection, header_parsing, runtime_config::RuntimeConfig};
use crate::prelude::*;
use crate::routine::{formatting::MessagesFormatting, info_channel::InfoChannel};
use std::{
    io::{self, Read, Result, Write},
    net::TcpListener,
    path::Path,
    process::exit,
    thread,
    time::Duration,
};

pub struct ServerInstance {
    info_channel: InfoChannel,
    connections: Vec<ClientConnection>,
    config: RuntimeConfig,
}

impl ServerInstance {
    pub fn new(info_channel: InfoChannel) -> Self {
        Self {
            info_channel,
            connections: vec![],
            config: RuntimeConfig::default(),
        }
    }

    pub async fn launch(&mut self, args: &[String]) -> Result<()> {
        self.config = RuntimeConfig::parse_arguments(args.to_vec()).unwrap();
        let error_marking = "Server Error".bright_red();
        if !RuntimeConfig::blz_exists(&self.config.manage_file) {
            let error_message = format!(
                "{}: {} not found; couldn't raise a server{}",
                error_marking,
                &self.config.manage_file,
                if args.is_empty() && self.config.manage_file == "main.manage.blz" {
                    "\n".to_owned()
                        + &String::from("Try to specify a path to the management file").into_hint()
                } else {
                    "".to_string()
                },
            );
            return Err(io::Error::new(ErrorKind::NotFound, error_message));
        }
        let listener = match TcpListener::bind(&self.config.address) {
            Ok(success) => {
                self.info_channel
                    .clone()
                    .send(format!(
                        "Gateway opened on {}",
                        self.config.address.yellow()
                    ))
                    .unwrap();
                success
            }
            Err(error) => {
                return Err(io::Error::new(
                    error.kind(),
                    format!(
                        "{}: {}; could not start",
                        error_marking,
                        match error.kind() {
                            ErrorKind::AddrInUse => {
                                "The address is already taken".to_string()
                            }
                            ErrorKind::PermissionDenied => {
                                "An attempt to bind the port was refused".to_string()
                            }
                            ErrorKind::InvalidInput => {
                                format!("{}: Invalid socket address", error_marking)
                            }
                            _ => {
                                format!("{}: {}", error_marking, error)
                            }
                        }
                    ),
                ));
            }
        };

        let info_channel = self.info_channel.clone();

        info_channel
            .clone()
            .send(format!(
                "Executing {}",
                Path::new(&self.config.manage_file)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .yellow()
            ))
            .unwrap();

        ctrlc::set_handler(move || {
            info_channel
                .clone()
                .send("\nInterrupted with ^C, shutdown".to_string())
                .unwrap();
            exit(0);
        })
        .expect("Error occured while setting ctrl+c handler");

        self.start_closed_connections_cleaner().await;
        self.handle_connections(listener).await?;
        Ok(())
    }

    async fn start_closed_connections_cleaner(&mut self) -> tokio::task::JoinHandle<()> {
        unsafe {
            let this = &mut *(self as *mut Self);
            tokio::spawn(async move {
                loop {
                    thread::sleep(Duration::from_secs(10));
                    this.connections
                        .retain(|connection: &ClientConnection| !connection.closed);
                }
            })
        }
    }

    async fn handle_connections(&mut self, listener: TcpListener) -> Result<()> {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let correct_password = &self.config.password;

            let mut buffer = [0; 1024];
            let mut request = String::new();

            let bytesize = stream.read(&mut buffer).unwrap();
            request.push_str(&String::from_utf8_lossy(&buffer[..bytesize]));

            let header = header_parsing::parse_header(request.clone()).unwrap();

            let client_password = header.get("password");
            if client_password.is_none() || correct_password != client_password.unwrap() {
                stream.write_all(b"HTTP/1.1 401 Unauthorized - Incorrect password\r\n\n")?;
                continue;
            } else if self.connections.len() + 1 > self.config.cons_limit as usize {
                stream.write_all(b"HTTP/1.1 503 Sessions Limit\r\n\n")?;
            }
            let new_session =
                ClientConnection::create(Local::now(), self.config.con_lifetime).await;
            let session_id = new_session.id;
            self.connections.push(new_session);
            let response_content = format!("{}", session_id);

            self.info_channel
                .clone()
                .send("A new connection's been created".to_string())
                .unwrap();

            stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 Ok\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\n{}",
                        response_content.len(),
                        response_content
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        Ok(())
    }
}
