use colored::Colorize;
use dotenvy::dotenv;
use regex::Regex;
use std::{
    env,
    ffi::OsStr,
    io::{self, Result},
    path::Path,
    time::Duration,
};

pub struct RuntimeConfig {
    pub address: String,
    pub manage_file: String,
    pub password: String,
    pub sessions_limit: u32,
    pub session_lifetime: Duration,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            address: "localhost:3306".to_string(),
            manage_file: "./main.manage.blz".to_string(),
            password: "password".to_string(),
            sessions_limit: 100,
            session_lifetime: Duration::from_secs(30),
        }
    }
}

impl RuntimeConfig {
    fn get_env_value(user_key: String) -> Option<String> {
        dotenv().expect("Environment file couldn't be found");

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

        if !blz_file.exists() || blz_file.extension().unwrap_or(OsStr::new("")) != "blz" {
            return false;
        }
        true
    }

    pub fn parse_arguments(console_args: Vec<String>) -> Result<Self> {
        let default = Self::default();
        let mut address = default.address;
        let mut password = default.password;
        let mut manage_file: String = "main.manage.blz".to_string();
        let mut sessions_limit: u32 = default.sessions_limit;
        let mut session_lifetime: Duration = default.session_lifetime;

        let manage_file_fullname_regex = Regex::new(r".*\.manage\.blz$").unwrap();

        for console_arg_index in 0..console_args.len() {
            let argument_keyword = &console_args[console_arg_index];
            if argument_keyword.chars().nth(0).unwrap() != '-' {
                if console_arg_index == 0 {
                    manage_file = if !manage_file_fullname_regex.is_match(argument_keyword) {
                        format!("{}.manage.blz", argument_keyword)
                    } else {
                        argument_keyword.to_string()
                    };
                };
                continue;
            }

            let argument_value = match console_args.get(console_arg_index + 1) {
                Some(value) => {
                    if let Some(env_value) =
                        RuntimeConfig::get_env_value(argument_keyword.to_string())
                    {
                        env_value
                    } else {
                        value.to_string()
                    }
                }
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "{}: The value for '{}' argument isn't provided",
                            "Input Error".bright_red(),
                            &argument_keyword[1..argument_keyword.len()]
                        ),
                    ));
                }
            };

            match argument_keyword.as_str() {
                "-address" => address.clone_from(&argument_value),
                "-password" => password.clone_from(&argument_value),
                "-session_lifetime" => session_lifetime.clone_from(&Duration::from_secs(
                    argument_value
                        .parse()
                        .expect("Provide the right amount of seconds a session lives"),
                )),
                "-sessions_limit" => sessions_limit.clone_from(
                    &argument_value
                        .parse()
                        .expect("Unable to parse 'sessions_limit' field due to the incorrect type"),
                ),
                _ => (),
            }
        }
        Ok(RuntimeConfig {
            address,
            manage_file,
            password,
            sessions_limit,
            session_lifetime,
        })
    }
}
