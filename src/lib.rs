use std::process;
#[cfg(test)]
mod tests;
// ANSI Escape Code Guide: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

#[allow(non_snake_case)]
struct Config {
    showHidden: bool,
    showAttributes: bool,
}

#[allow(non_snake_case)]
pub fn run(/*os: &str,*/ path: &str, showHidden: bool, showAttributes: bool){

    let files = std::fs::read_dir(path).unwrap_or_else(|error: std::io::Error| {
        println!("Error: {error}");
        process::exit(1);
    });

    let currConfig = Config{showHidden, showAttributes};

    platform::printDir(files, currConfig);
}

pub fn help() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("neodir v{}\n", VERSION);
    println!("Usage: neodir [DIRECTORY] [OPTIONS]\n");
    // Options
    println!("OPTIONS:");
    println!("\t-a: Show file attributes (Windows Only)");
    println!("\t-h: Show hidden files (Windows Only)");
    println!("\nNote: Options can only be used if you provide a directory.\nFor the current directory, use '.'");
}

// Conditional Compiling code written with the help of Jmb on StackOverflow
// https://stackoverflow.com/a/77805203/12884111
#[allow(non_snake_case)]
#[cfg (target_os = "windows")]
mod platform {
    use std::fs::Metadata;
    use std::{fs, ops::Mul};
    use std::os::windows::fs::MetadataExt;
    use chrono::format::{DelayedFormat, StrftimeItems};
    use chrono::{Utc, DateTime, Local};
    // use windows::Win32; 
    use std::process::Command;

    pub fn printDir(currDir: fs::ReadDir, currConfig: super::Config){
        println!("");
        printStorageSpace();
        println!("");

        print!("\x1b[34m{:50} {:15} {:15} {:30}", "File", "Size (Bytes)", "Read-Only", "Last Modified");
        if currConfig.showAttributes {
            print!("{}","File Attributes");
        }
        println!("\x1b[0m");
        
        'infoDump: for file in currDir {
    
            let mut fileSpacing = 50;

            let file = file.unwrap();
            let currentFilePath = &file.path();
            
            // Retrieve file Metadata
            let fileMetadata = match fs::metadata(&currentFilePath) {
                Err(error) =>  match error.kind() {
                    std::io::ErrorKind::PermissionDenied => continue 'infoDump,
                    _ => panic!("An error occured: {error}")
                }
                Ok(fileMetadata) => fileMetadata
            };

            // Check the attribute of the current file/folder
            if checkWinFileAttribute(&fileMetadata,WinFileAttribute::Hidden) && !currConfig.showHidden {
                continue 'infoDump;
            }
            
            // Get current file/folder name
            let fileName;
            if fileMetadata.is_dir() {
                // For coloured file names
                fileName =format!("\x1b[95m{}\x1b[0m",&file.file_name()
                                            .into_string()
                                            .unwrap()
                );
                // Since the folder names are coloured and use invisible colours, we have
                // to increase the spacing so everything stays in-line
                fileSpacing = 59;
            } else {
                fileName = file.file_name().into_string().unwrap()
            };
            
            // Folders should not show their sizes
            let fileSize = if fileMetadata.is_file() { 
                fileMetadata.file_size().to_string()
            } else { 
                "".to_string() 
            };
    
            // Perform Microsoft Epoch to Unix Epoch conversion
            let lastModifiedSecs = ((fileMetadata.last_write_time()/10000000) - 11644473600) as i64;
            let lastModified = getHumanTime(&lastModifiedSecs);
            
            print!("{:fileSpacing$} {:15} {:15} {:30}", fileName, fileSize, fileMetadata.permissions().readonly(), lastModified);
            if currConfig.showAttributes {
                print!("{}",fileMetadata.file_attributes())
            }
            println!("");
        }
    }

    /// Returns human-readable time for the Unix Time entered
    fn getHumanTime(seconds: &i64) -> DelayedFormat<StrftimeItems>{
        return DateTime::<Utc>::from_timestamp(*seconds,0)
        .unwrap().with_timezone(&Local)
        .format("%d/%m/%Y %H:%M");
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
    /// Returns the available space of the C: drive in bytes
    fn getAvailableSpace() -> u64 {
        let availInfo = Command::new("wmic")
                                    .arg("LogicalDisk")
                                    .arg("Get")
                                    .arg("freespace")
                                    .output()
                                    .expect("Error");
        let result = String::from_utf8(availInfo.stdout).expect("Error");
        let mut availSpace = result.lines();
        let availSpaceB = availSpace.nth(1).expect("uhoh").trim().parse::<u64>().unwrap();

        return availSpaceB;
    }            
    /// Returns the available space on the C: drive, as a percentage decimal
    fn getAvailableSpacePer() -> f32 {
        let percentage: f32 = getAvailableSpace() as f32 / getDriveSize() as f32;
        return percentage;
    }
    
    fn printStorageSpace(){

        let endSeq = "\x1b[0m";

        let freeSpace = getAvailableSpacePer().mul(50.0) as usize;
        let usedSpace = 50 - freeSpace;

        let colour = if freeSpace > 0.2.mul(50.0) as usize {
            "\x1b[32m"
        } else {
            "\x1b[31m"
        };

        println!("{colour}C: {:█<usedSpace$}{:░<freeSpace$} {}% Free {endSeq}","","",freeSpace*2);
    }

    #[allow(dead_code)]
    /// An enum for storing Windows File Attribute values.
    /// Values taken from: https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
    enum WinFileAttribute {
        ReadOnly    = 0x00000001,
        Hidden      = 0x00000002,
        Archive     = 0x00000020
    }

    fn checkWinFileAttribute(currFile: &Metadata, attribute: WinFileAttribute) -> bool{
        let attributeVal = attribute as u32;
        return currFile.file_attributes() & attributeVal == attributeVal;
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
    use chrono::format::{DelayedFormat, StrftimeItems};

    pub fn printDir(currDir: fs::ReadDir, currConfig: super::Config) {
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
    
            let lastModifiedSecs = fileMetadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
            let lastModified =  getHumanTime(&lastModifiedSecs);
    
            println!("{:fileSpacing$} {:15} {:15} {:30}", fileName, fileSize, fileMetadata.permissions().readonly(), lastModified);
        }
    }   

    fn getHumanTime(seconds: &i64) -> DelayedFormat<StrftimeItems>{
        return DateTime::<Utc>::from_timestamp(*seconds,0)
        .unwrap().with_timezone(&Local)
        .format("%d/%m/%Y %H:%M");
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
    use chrono::format::{DelayedFormat, StrftimeItems};
    use std::fs;

    fn printDir(currDir: fs::ReadDir, currConfig: super::Config) {
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
    
            let lastModifiedSecs = fileMetadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
            let lastModified =  getHumanTime(&lastModifiedSecs);
    
            println!("{:fileSpacing$} {:15} {:15} {:30}", fileName, fileSize, fileMetadata.permissions().readonly(), lastModified);
        }
    }   

    fn getHumanTime(seconds: &i64) -> DelayedFormat<StrftimeItems>{
        return DateTime::<Utc>::from_timestamp(*seconds,0)
        .unwrap().with_timezone(&Local)
        .format("%d/%m/%Y %H:%M");
    }
}