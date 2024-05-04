use std::collections::HashMap;

pub fn parse_header(response: String) -> Option<HashMap<String, String>> {
    let mut header: HashMap<String, String> = HashMap::new();

    for line in response.lines().skip(1) {
        if line.is_empty() {
            break;
        }

        let mut parse: Vec<&str> = line.split(':').collect();
        if parse[1].starts_with(' ') {
            parse[1] = &parse[1][1..];
        }

        header.insert(parse[0].to_string(), parse[1].to_string());
    }

    Some(header)
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
