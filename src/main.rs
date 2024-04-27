use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

mod lexer;
use lexer::Lexer;

const OFFICIAL_REPOSITORY: &str = "https://github.com/Reeliks/blaze";

fn create_db_structure(path_to_db: &str, creation_process_printing: bool) -> io::Result<()> {
    let mut db_workdir_path_buf = PathBuf::from(&path_to_db);
    db_workdir_path_buf.push("datablaze");
    create_db_folders(&db_workdir_path_buf)?;
    
    let is_manage_file_created: bool = create_manage_file(&db_workdir_path_buf)?;
    if creation_process_printing {
        if is_manage_file_created {
            println!("manage.blz has been created")
        }
        else {
            println!("manage.blz already exists; skipping...");
        }
        println!("\nA new datablaze has been structured. Use 'blaze --help' to see the commands.\nTo contribute the development process, check out the official repository:\n{}", OFFICIAL_REPOSITORY);
    }
    
    Ok(())
}

fn create_db_folders(db_path_buf: &PathBuf) -> io::Result<()> {
    for folder in ["data", "model"] {
        let mut cloned_db_path_buf = db_path_buf.clone();
        cloned_db_path_buf.push(folder);
        fs::create_dir_all(cloned_db_path_buf)?;
    };
    Ok(())  
}

fn create_manage_file(path_to_db_buf: &PathBuf) -> io::Result<bool>{
    let mut managing_file_path_buf = path_to_db_buf.clone();
    managing_file_path_buf.push("manage.blz");
    let _ = recreate_directories(&managing_file_path_buf.clone());
    if let Err(e) = fs::metadata(managing_file_path_buf.to_str().unwrap()) {
        match e.kind() {
            std::io::ErrorKind::NotFound => {
                let mut managing_file = File::create(&mut managing_file_path_buf)?;
                let _ = managing_file.write_all(b"@manage (\n\tmax_connections = 3,\n\tworkdir = \"./\"\n);\n\n@backups \"./backups\";\n\n@attach \"./data/main\";");
                return Ok(true);
            }
            _ => {
                panic!("{}", e);
            }
        };
    };
    Ok(false)
}

fn recreate_directories(path_buf: &PathBuf) -> io::Result<()> {
    if let Some(parent_dir) = path_buf.parent() {
        fs::create_dir_all(parent_dir)?;
    };
    Ok(())
}

pub fn create_db_with_console() -> io::Result<()> {
    let mut path = String::new();
    println!("Specify a path to a datablaze");
    _ = io::stdin().read_line(&mut path);
    let path = path.trim();
    create_db_structure(path, true)?;

    Ok(())
}

fn print_help_section() -> io::Result<()>
{
    println!("Blaze Db 0.0.1a\nAvailable commands:\nlex\t- try the first version of Blaze Language Lexer\ncreate\t- create a new datablaze");
    Ok(())
}

fn analyze_lexically () -> io::Result<()> 
{
    let mut code_to_parse = String::new();
    std::io::stdin().read_line(&mut code_to_parse)?;
    code_to_parse = code_to_parse.trim().to_string();
    Lexer::new(code_to_parse, "Buffer".to_string()).analyze()?;
    Ok(())
}

fn handle_arguments() -> io::Result<()> 
{
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() == 2 
    {
        match args[1].as_str() 
        {
            "create" => create_db_with_console()?,
            "lexer" => analyze_lexically()?,
            _ => todo!()
        }
    }
    else
    {
        print_help_section()?;
    }
    Ok(())
}

fn main() -> io::Result<()> 
{
    handle_arguments()?;
    Ok(())
}
