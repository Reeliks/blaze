use dotenvy::dotenv;
use regex::Regex;
use std::env;

pub struct Config;

impl Config {
    pub fn value(user_key: String) -> Option<String> {
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
}
