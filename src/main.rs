use std::env;
use std::process;

#[allow(non_snake_case)]
fn main() {

    let _args: Vec<String> = env::args().collect();
    // println!("Executable is at: {}", _args[0]);

    if _args.contains(&String::from("--help")) {
        neodir::help();
        process::exit(0);
    } else {
        // I want to do this with Result to detect if there are any arguments at all, but I dunno how
        // TODO: Do this with Results
        let directedLoc= if _args.len() >= 2 {
            &_args[1]
        } else {
            "."
        };
        neodir::run(directedLoc);
        process::exit(0);
    }
}