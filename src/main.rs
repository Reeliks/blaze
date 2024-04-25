use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use regex::Regex;

fn create_db_structure(path_to_db: &str) -> io::Result<()> {
    let mut db_workdir_path_buf = PathBuf::from(&path_to_db);
    db_workdir_path_buf.push("datablaze");
    create_db_folders(&db_workdir_path_buf)?;
    create_manage_file(&db_workdir_path_buf)?;
    Ok(())
}


fn create_db_folders(db_path_buf: &PathBuf) -> io::Result<()> {
    for folder in ["data", "model", "backups"] {
        let mut cloned_db_path_buf = db_path_buf.clone();
        cloned_db_path_buf.push(folder);
        fs::create_dir_all(cloned_db_path_buf)?;
    };
    Ok(())  
}

fn create_manage_file(path_to_db_buf: &PathBuf) -> io::Result<()>{
    let mut managing_file_path_buf = path_to_db_buf.clone();
    managing_file_path_buf.push("manage.blz");
    let _ = recreate_directories(&managing_file_path_buf.clone());
    if let Err(e) = fs::metadata(managing_file_path_buf.to_str().unwrap()) {
        match e.kind() {
            std::io::ErrorKind::NotFound => {
                let mut managing_file = File::create(&mut managing_file_path_buf)?;
                let _ = managing_file.write_all(b"@manage (\n\tmax_connections = 3,\n\tworkdir = \"./\"\n);\n\n@backups \"./backups\";\n\n@attach \"./data/main\";");
            }
            _ => {
                panic!("{}", e);
            }
        };
    };
    Ok(())
}

fn recreate_directories(path_buf: &PathBuf) -> io::Result<()> {
    if let Some(parent_dir) = path_buf.parent() {
        fs::create_dir_all(parent_dir)?;
    };
    Ok(())
}

pub fn create_db_with_user_input() -> io::Result<()> {
    let mut path = String::new();
    println!("Specify a path to a new database");
    _ = io::stdin().read_line(&mut path);
    let path = path.trim();
    create_db_structure(path)?;

    Ok(())
}

struct RegexForParsingBlazeCode {
    variable_name: Regex,
    
}

impl RegexForParsingBlazeCode {
    fn new () -> Self {
        let variable_name = Regex::new(r"^[a-zA-Z]\w*$").unwrap();
    
        RegexForParsingBlazeCode { variable_name }
    }
}


pub fn parse_for_db_tokens_first_version(string_to_parse: &str) -> io::Result<Vec<String>> {
    let primitive_tokens= vec!["{", "}", "=", ";"];
    
    let mut content = string_to_parse.to_string();
    let mut tokens: Vec<String> = Vec::new();
    let mut buf = String::new();

    let mut inside_string = false;
    let mut inside_comment = false;
    let mut deepness = 0;

    while !content.is_empty() {
        let first_char: char = content.remove(0);
        let inside_text: bool = inside_string || inside_comment;
        if Regex::new(r"\w").unwrap().is_match(&buf) && !inside_text
        {
            let matches = RegexForParsingBlazeCode::new().variable_name.is_match(&buf); 
            if !matches
            {
                panic!("Invalid variable name: {}", buf);
            } 
            let letter_regex = Regex::new(r"\w").unwrap();
            let is_next_char_part_of_variable = letter_regex.is_match(&first_char.to_string());
            if !is_next_char_part_of_variable
            {
                tokens.push(buf.to_string());
                buf.clear();
            }
        };
        if primitive_tokens.contains(&buf.as_str()) && !inside_text
        { tokens.push(buf.to_string()); if buf == "{" { deepness += 1; } else if buf == "}" { deepness -= 1; }; if deepness < 0 { panic!("Closing bracket when it's not needed"); } buf.clear(); }; if buf == "//" && !inside_text { inside_comment = true; } else if first_char == '"' && !inside_comment { inside_string = !inside_string; buf.push(first_char); if !inside_string { tokens.push(buf.to_string()); buf.clear(); } } else if first_char == '\n' && !inside_string 
        {
            inside_comment = false;
            buf.clear();
        }
        else if !vec![' ', '\t'].contains(&first_char) || inside_text
        {
            println!("push: {}", first_char);
            buf.push(first_char);
            
        }
        else 
        {
            println!("clean: {}", buf);
            buf.clear();
        };
    };

    if deepness != 0 {
        panic!("Uncompleted scope");
    };

    println!("\nresults");
    for token in tokens.iter() {
        println!("{}", token);
    };
    Ok(tokens)
}


fn main() -> io::Result<()> {
    let _ = create_db_with_user_input()?;
    Ok(())
}
