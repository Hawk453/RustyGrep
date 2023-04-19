#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        //args[0] is binary name, which dont need for the proejct. We will ignore it.
        let query = args[1].clone(); //String needed to be searched
        let file_path = args[2].clone(); //Input Text File

        Ok(Config { query, file_path})
    }
}


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

    // Reading a file
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    
}
