use std::process;

pub fn run(/*os: &str,*/ path: &str){

    let files = std::fs::read_dir(path).unwrap_or_else(|error: std::io::Error| {
        println!("Error: {error}");
        process::exit(1);
    });

    platform::printDir(files);
}

// Conditional Compiling code written with the help of Jmb on StackOverflow
// https://stackoverflow.com/a/77805203/12884111
#[allow(non_snake_case)]
#[cfg (target_os = "windows")]
mod platform {
    use std::fs;
    use std::os::windows::fs::MetadataExt;
    use chrono::{Utc, DateTime, Local}; 

    pub fn printDir(currDir: fs::ReadDir){

        println!("\x1b[34m{:50} {:15} {:15} {:30}\x1b[0m", "File", "Size (Bytes)", "Read-Only", "Last Modified");
    
        'infoDump: for file in currDir {
    
            let mut fileSpacing = 50;

            let file = file.unwrap();
            let currentFilePath = &file.path();
            
            let fileMetadataResult = fs::metadata(&currentFilePath);
            let fileMetadata = match fileMetadataResult {
                Err(error) =>  match error.kind() {
                    std::io::ErrorKind::PermissionDenied => continue 'infoDump,
                    _ => panic!("An error occured: {error}")
                }
                Ok(fileMetadata) => fileMetadata
            };
    
            let fileName = if fileMetadata.is_dir() {
                // For coloured file names
                format!("\x1b[95m{}\x1b[0m",&file.file_name().into_string().unwrap())
            } else {
                file.file_name().into_string().unwrap()
            };
    
            // TODO: Optimise this
            if fileName != file.file_name().into_string().unwrap(){
                fileSpacing = 59;
            }
    
            let fileSize = if fileMetadata.is_file() { 
                fileMetadata.file_size().to_string()
            } else { 
                "".to_string() 
            };
    
            // Perform Microsoft Epoch to Unix Epoch conversion
            let lastModifiedSecs = (fileMetadata.last_write_time()/10000000) - 11644473600;
            let lastModified = DateTime::<Utc>::from_timestamp(lastModifiedSecs as i64,0)
                                                                .unwrap().with_timezone(&Local)
                                                                .format("%d/%m/%Y %H:%M");
    
            println!("{:fileSpacing$} {:15} {:15} {:30}", fileName, fileSize, fileMetadata.permissions().readonly(), lastModified);
        }
    }
    
}

#[allow(non_snake_case)]
#[cfg (target_os = "linux")]
#[allow(unused_imports)]
mod platform {
    
    use chrono::{Utc, DateTime, Local};
    use std::fs;
    use std::os::linux;

    pub fn printDir(currDir: fs::ReadDir) {
        println!("\x1b[34m{:50} {:15} {:15} {:30}\x1b[0m", "File", "Size (Bytes)", "Read-Only", "Last Modified");
    
        'infoDump: for file in currDir {
    
            let mut fileSpacing = 50;
    
            let file = file.unwrap();
            let currentFilePath = &file.path();
            
            let fileMetadataResult = fs::metadata(&currentFilePath);
            let fileMetadata = match fileMetadataResult {
                Err(error) =>  match error.kind() {
                    std::io::ErrorKind::PermissionDenied => continue 'infoDump,
                    _ => panic!("An error occured: {error}")
                }
                Ok(fileMetadata) => fileMetadata
            };
    
            let fileName = if fileMetadata.is_dir() {
                // For coloured file names
                format!("\x1b[95m{}\x1b[0m",&file.file_name().into_string().unwrap())
            } else {
                file.file_name().into_string().unwrap()
            };
    
            // TODO: Optimise this
            if fileName != file.file_name().into_string().unwrap(){
                fileSpacing = 59;
            }
    
            let fileSize = if fileMetadata.is_file() { 
                fileMetadata.len().to_string()
            } else { 
                "".to_string() 
            };
    
            let lastModifiedSecs = fileMetadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
            let lastModified =  DateTime::<Utc>::from_timestamp(lastModifiedSecs as i64,0)
                                .unwrap().with_timezone(&Local).format("%d/%m/%Y %H:%M");
    
            println!("{:fileSpacing$} {:15} {:15} {:30}", fileName, fileSize, fileMetadata.permissions().readonly(), lastModified);
        }
    }   
}

#[allow(non_snake_case)]
#[cfg (not (any (target_os = "windows", target_os = "linux")))]
mod platform {
    
    use chrono::{Utc, DateTime, Local};
    use std::fs;

    fn printDir(currDir: fs::ReadDir) {
        println!("\x1b[34m{:50} {:15} {:15} {:30}\x1b[0m", "File", "Size (Bytes)", "Read-Only", "Last Modified");
    
        'infoDump: for file in currDir {
    
            let mut fileSpacing = 50;
    
            let fileMetadataResult = fs::metadata(&currentFilePath);
            let fileMetadata = match fileMetadataResult {
                Err(error) =>  match error.kind() {
                    std::io::ErrorKind::PermissionDenied => continue 'infoDump,
                    _ => panic!("An error occured: {error}")
                }
                Ok(fileMetadata) => fileMetadata
            };
    
            let fileName = if fileMetadata.is_dir() {
                // For coloured file names
                format!("\x1b[95m{}\x1b[0m",&fileDirEntry.file_name().into_string().unwrap())
            } else {
                file.file_name().into_string().unwrap()
            };
    
            // TODO: Optimise this
            if fileName != file.file_name().into_string().unwrap(){
                fileSpacing = 59;
            }
    
            let fileSize = if fileMetadata.is_file() { 
                fileMetadata.len().to_string()
            } else { 
                "".to_string() 
            };
    
            let lastModifiedSecs = fileMetadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
            let lastModified =  DateTime::<Utc>::from_timestamp(lastModifiedSecs as i64,0)
                                .unwrap().with_timezone(&Local).format("%d/%m/%Y %H:%M");
    
            println!("{:fileSpacing$} {:15} {:15} {:30}", fileName, fileSize, fileMetadata.permissions().readonly(), lastModified);
        }
    }   
}