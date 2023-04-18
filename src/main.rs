#![allow(non_snake_case)]

use std::env;

fn main() {
    // Utilising String, not OsString, cause of simplicity and portability.
    // OsString values differ per platform and are more complex to work with than String values.
    let args: Vec<String> = env::args().collect();

    //args[0] is binary name, which dont need for the proejct. We will ignore it.

    let query = &args[1]; //String needed to be searched
    let file_path = &args[2]; //Input Text File

    println!("Searchin for {query}");
    println!("In text file {file_path}");
    
}
