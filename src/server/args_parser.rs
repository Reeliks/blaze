use crate::server::config::Config;

#[derive(Debug)]
pub struct Args {
    pub ip: String,
    pub port: String,
    pub blz_file: String,
    pub password: String,
}

impl Args {
    pub fn parse(args: Vec<String>) -> Option<Self> {
        let default = Self::default();
        let mut ip = default.ip.clone();
        let mut port = default.port.clone();
        let mut blz_file = default.blz_file.clone();
        let mut password = default.password.clone();

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
                "-ip" => ip.clone_from(&value),
                "-port" => port.clone_from(&value),
                "-blz_file" => blz_file.clone_from(&value),
                "-password" => password.clone_from(&value),
                _ => (),
            }
        }
        Some(Args {
            ip,
            port,
            blz_file,
            password,
        })
    }

    fn default() -> Self {
        Args {
            ip: "localhost".to_string(),
            port: "3306".to_string(),
            blz_file: "./".to_string(),
            password: "password".to_string(),
        }
    }
}
