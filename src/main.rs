use std::env;
use std::fs;
use std::process;

use neodir::System;

#[allow(non_snake_case)]
fn main() {

    let _args: Vec<String> = env::args().collect();
    // println!("Executable is at: {}", _args[0]);

    // I want to do this with Result to detect if there are any arguments at all, but I dunno how
    // TODO: Do this with Results
    let directedLoc= if _args.len() >= 2 {
        &_args[1]
    } else {
        "."
    };

    let files = fs::read_dir(directedLoc).unwrap_or_else(|error: std::io::Error| {
        println!("Error: {error}");
        process::exit(1);
    });

    let userSys: System = System{os: env::consts::OS, files};
    neodir::printDir(userSys);

}