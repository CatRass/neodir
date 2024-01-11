use std::{fs, time::UNIX_EPOCH};
use chrono::{Utc, DateTime, Local};
use std::process;

pub fn run(os: &str, path: &str){
    let files = fs::read_dir(path).unwrap_or_else(|error: std::io::Error| {
        println!("Error: {error}");
        process::exit(1);
    });

    // NOTE: This code is useless in its current state.
    // As you cannot use std::os::windows on other platforms,
    // the import causes compilation errors on Linux (and possibly other platforms).
    // Thus, I need a new solution to making the program compilable on different OSes
    // using the same code.
    if os == "windows" {
        printDir_win(files);
    } else {
        println!("{}OS not recognised: defaulting to std::fs methods{}","\x1b[31m","\x1b[0m");
        printDir(files);
    }
}

#[allow(non_snake_case)]
pub fn printDir_win(currDir: fs::ReadDir){
    
    use std::os::windows::fs::MetadataExt;

    println!("\x1b[34m{:50} {:15} {:15} {:30}\x1b[0m", "File", "Size (Bytes)", "Read-Only", "Last Modified");

    for file in currDir {

        let mut fileSpacing = 50;

        let currentFilePath = file.unwrap().path();
        let fileMetadata = fs::metadata(&currentFilePath).unwrap();

        let fileName = if fileMetadata.is_dir() {
            // For coloured file names
            format!("\x1b[95m{}\x1b[0m",&currentFilePath.display())
            // currentFilePath.display().to_string()
        } else {
            currentFilePath.display().to_string()
        };

        // TODO: Optimise this
        if fileName != currentFilePath.display().to_string(){
            fileSpacing = 59;
        }

        let fileSize = if fileMetadata.is_file() { 
            fileMetadata.file_size().to_string()
        } else { 
            "".to_string() 
        };

        // Perform Microsoft Epoch to Unix Epoch conversion
        let lastModifiedSecs = (fileMetadata.last_write_time()/10000000) - 11644473600;
        let lastModified = DateTime::<Utc>::from_timestamp(lastModifiedSecs as i64,0).unwrap().with_timezone(&Local).format("%d/%m/%Y %H:%M");

        println!("{:fileSpacing$} {:15} {:15} {:30}", fileName, fileSize, fileMetadata.permissions().readonly(), lastModified);
    }
}

#[allow(non_snake_case)]
fn printDir(currDir: fs::ReadDir) {
    println!("\x1b[34m{:50} {:15} {:15} {:30}\x1b[0m", "File", "Size (Bytes)", "Read-Only", "Last Modified");

    for file in currDir {

        let mut fileSpacing = 50;

        let currentFilePath = file.unwrap().path();
        let fileMetadata = fs::metadata(&currentFilePath).unwrap();

        let fileName = if fileMetadata.is_dir() {
            // For coloured file names
            format!("\x1b[95m{}\x1b[0m",&currentFilePath.display())
            // currentFilePath.display().to_string()
        } else {
            currentFilePath.display().to_string()
        };

        // TODO: Optimise this
        if fileName != currentFilePath.display().to_string(){
            fileSpacing = 59;
        }

        let fileSize = if fileMetadata.is_file() { 
            fileMetadata.len().to_string()
        } else { 
            "".to_string() 
        };

        let lastModifiedSecs = fileMetadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let lastModified = DateTime::<Utc>::from_timestamp(lastModifiedSecs as i64,0).unwrap().with_timezone(&Local).format("%d/%m/%Y %H:%M");

        println!("{:fileSpacing$} {:15} {:15} {:30}", fileName, fileSize, fileMetadata.permissions().readonly(), lastModified);
    }
}