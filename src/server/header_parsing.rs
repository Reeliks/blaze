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

pub fn remove_empty_line(response: String) -> Option<String> {
    let mut empty_line_found = false;
    let mut data = String::new();

    for line in response.lines() {
        if empty_line_found {
            data.push_str(line);
        }

        if line.is_empty() && !empty_line_found {
            empty_line_found = true;
        }
    }

    if data.is_empty() {
        return None;
    }

    Some(data)
}
