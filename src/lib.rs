#![allow(non_snake_case)]

use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        //args[0] is binary name, which we dont need for the proejct. We will ignore it.
        let query = args[1].clone(); //String needed to be searched
        let file_path = args[2].clone(); //Input Text File

        Ok(Config {query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Reading a file
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }    

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}