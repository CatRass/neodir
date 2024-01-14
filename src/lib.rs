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
    use std::process::Command;

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
                format!("\x1b[95m{}\x1b[0m",&file.file_name()
                                            .into_string()
                                            .unwrap()
                )
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
    
    /// Returns the Size of the C: drive in bytes
    fn getDriveSize() -> u64{
        let diskInfo = Command::new("wmic")
                                .arg("diskdrive")
                                .arg("get")
                                .arg("size")
                                .output()
                                .expect("Error");
        let result = String::from_utf8(diskInfo.stdout).expect("Error");
        let mut driveSize = result.lines();

        let driveSizeB = driveSize.nth(1).expect("uhoh").trim().parse::<u64>().unwrap();

        return driveSizeB;
    }
    
}

#[allow(non_snake_case)]
#[cfg (target_os = "linux")]
#[allow(unused_imports)]
mod platform {
    
    use chrono::{Utc, DateTime, Local, OutOfRange};
    use std::fs;
    use std::os::linux;
    use std::process::{Command,Stdio};

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
    
    fn getDriveSize(){
        let driveSizeCmd =  Command::new("df")
                            .arg("-T")
                            .arg("/dev/sda")
                            .stdout(Stdio::piped())
                            .spawn()
                            .expect("Error!");

        let dfResult = driveSizeCmd.stdout.expect("Error");
        
        // TODO: Fix the error caused by the last arg.
        // The error is:
        // awk: cmd. line:1: '{print $1}'
        // awk: cmd. line:1: ^ invalid char ''' in expression
        // Relevant docs: https://doc.rust-lang.org/std/process/index.html#handling-io
        let pipeResult = Command::new("awk")
                        .arg("-F ' '")
                        .arg("'{print $1}'")
                        .stdin(Stdio::from(dfResult))
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Error");
        
        let varStr = pipeResult.wait_with_output().unwrap(); 
        println!("{}",String::from_utf8(varStr.stdout).unwrap());
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