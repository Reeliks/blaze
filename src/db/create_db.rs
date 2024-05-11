use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::{Path, PathBuf};

pub const OFFICIAL_REPOSITORY: &str = "https://github.com/Reeliks/blaze";

pub fn create_db_structure(path_to_db: &str) -> Result<()> {
    let mut db_workdir_path_buf = PathBuf::from(&path_to_db);
    db_workdir_path_buf.push("datablaze");
    create_db_folders(&db_workdir_path_buf)?;

    let is_manage_file_created: bool = create_manage_file(&db_workdir_path_buf)?;
    if is_manage_file_created {
        println!("manage.blz has been created")
    } else {
        println!("manage.blz already exists; skipping...");
    }
    println!("\nA new datablaze has been structured. Use 'blaze --help' to see the commands.\nTo contribute the development process, check out the official repository:\n{}", OFFICIAL_REPOSITORY);

    Ok(())
}

fn create_db_folders(db_path_buf: &Path) -> Result<()> {
    for folder in ["data", "model"] {
        let mut cloned_db_path_buf = db_path_buf.to_path_buf();
        cloned_db_path_buf.push(folder);
        fs::create_dir_all(cloned_db_path_buf)?;
    }
    Ok(())
}

fn create_manage_file(path_to_db_buf: &Path) -> Result<bool> {
    let mut managing_file_path_buf = path_to_db_buf.to_path_buf();
    let manage_file_content = br#"manage (
    tmax_connections = 3,
    work_dir = "/",
    backups_dir = "backups/"
);

attach "/data/main;"#;

    managing_file_path_buf.push("manage.blz");
    if let Err(e) = fs::metadata(managing_file_path_buf.to_str().unwrap()) {
        match e.kind() {
            std::io::ErrorKind::NotFound => {
                let mut managing_file = File::create(&mut managing_file_path_buf)?;
                managing_file.write_all(manage_file_content)?;
                return Ok(true);
            }
            _ => {
                panic!("{}", e);
            }
        };
    };
    Ok(false)
}