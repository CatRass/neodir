use std::{fs, time::UNIX_EPOCH};
use chrono::{Utc, DateTime, Local};

#[allow(non_snake_case)]
/// Struct for storing system information
pub struct System<'a> {
    /// Users current operating system
    pub os: &'a str,
    /// All the files in the current directory
    pub files: fs::ReadDir,
}

#[allow(non_snake_case)]
pub fn printDir(currentSys: System){

    println!("\x1b[34m{:50} {:15} {:15} {:30}\x1b[0m", "File", "Size (Bytes)", "Read-Only", "Last Modified");

    for file in currentSys.files {

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