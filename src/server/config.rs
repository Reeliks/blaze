use dotenvy::dotenv;
use regex::Regex;
use std::{env, ffi::OsStr, path::Path};

pub struct Config {
    pub host: String,
    pub port: String,
    pub manager_file: String,
    pub password: String,
}

impl Config {
    fn value(user_key: String) -> Option<String> {
        dotenv().expect(".env file not found");

        let user_key = Regex::new(r"env\.(\w+)")
            .unwrap()
            .captures(&user_key)?
            .get(1)?
            .as_str();

        env::vars()
            .filter(|(key, _)| *key == user_key)
            .map(|(_, value)| value)
            .next()
    }

    pub fn blz_exists(path: &String) -> bool {
        let blz_file = Path::new(&path);

        if !blz_file.exists() 
        || blz_file.extension().unwrap_or(OsStr::new("")) != "blz" {
            return false
        }
        true
    }

    pub fn parse_arguments(args: Vec<String>) -> Option<Self> {
        let default = Self::default();
        let mut host = default.host;
        let mut port = default.port;
        let mut manager_file = default.manager_file;
        let mut password = default.password;

        for arg in 0..args.len() {
            let str = &args[arg];
            if str.chars().nth(0)? != '-' {
                continue;
            }

            let arg = args.get(arg + 1)?;
            let value = if let Some(parse_value) = Config::value(arg.to_string()) {
                parse_value
            } else {
                arg.to_string()
            };

            match str.as_str() {
                "-host" => host.clone_from(&value),
                "-port" => port.clone_from(&value),
                "-blz_file" => manager_file.clone_from(&value),
                "-password" => password.clone_from(&value),
                _ => (),
            }
        }
        Some(Config {
            host,
            port,
            manager_file,
            password,
        })
    }

    fn default() -> Self {
        Config {
            host: "localhost".to_string(),
            port: "3306".to_string(),
            manager_file: "./db/datablaze/manage.blz".to_string(),
            password: "password".to_string(),
        }
    }
}
