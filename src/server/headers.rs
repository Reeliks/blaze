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

pub fn del_headers(response: String) -> Option<String> {
    let mut is_empty_line = false;

    for line in response.lines() {
        if is_empty_line {
            return Some(line.to_string());
        }

        if line.is_empty() {
            is_empty_line = true;
        }
    }

    None
}
