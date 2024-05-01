use crate::server::args_parser::Args;
use std::io;

pub fn server_run(args: Vec<String>) -> io::Result<()> {
    println!("{:#?}", Args::parse(args).unwrap());

    Ok(())
}
