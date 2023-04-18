#![allow(non_snake_case)]

use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String
}



fn parse_config(args: &[String]) -> Config {

    //args[0] is binary name, which dont need for the proejct. We will ignore it.
    let query = args[1].clone(); //String needed to be searched
    let file_path = args[2].clone(); //Input Text File

    Config { query, file_path }
}

fn main() {
    // Utilising String, not OsString, cause of simplicity and portability.
    // OsString values differ per platform and are more complex to work with than String values.
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searchin for '{}'", config.query);
    println!("In text file {}", config.file_path);

    // Reading a file
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    
}
