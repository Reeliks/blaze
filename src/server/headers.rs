use std::io;

pub struct Header {
    token: String,
    value: String,
}

impl Header {
    fn header_parser(response: String) -> std::io::Result<Vec<Self>> {
        let mut all = vec![];

        for line in response.lines().skip(1) {
            if line.is_empty() {
                break;
            }

            let mut parse: Vec<&str> = line.split(':').collect();
            if parse[1].starts_with(' ') {
                parse[1] = &parse[1][1..];
            }

            all.push(Header {
                token: parse[0].to_string(),
                value: parse[1].to_string(),
            })
        }

        Ok(all)
    }

    pub fn get_value(response: String, token: String) -> io::Result<String> {
        let headers = Self::header_parser(response)?;

        for header in headers {
            if header.token == token {
                return Ok(header.value);
            }
        }

        Ok("".to_string())
    }

    pub fn del_headers(response: String) -> String {
        let mut is_empty_line = false;
        let mut result = "".to_string();

        for line in response.lines() {
            if line.is_empty() {
                is_empty_line = true;
            }

            if is_empty_line {
                result = line.to_string();
            }
        }

        result
    }
}
