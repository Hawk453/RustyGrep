#![allow(non_snake_case)]

use std::env;
use std::process;

use RustyGrep::Config;


fn main() {
    // Utilising String, not OsString, cause of simplicity and portability.
    // OsString values differ per platform and are more complex to work with than String values.
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem Parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searchin for '{}'", config.query);
    println!("In text file {}", config.file_path);

    if let Err(e) = RustyGrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
    
}
