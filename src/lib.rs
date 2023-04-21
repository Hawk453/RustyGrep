#![allow(non_snake_case)]

use std::error::Error;
use std::{fs, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
        ) -> Result<Config, &'static str> {
        
        //args[0] is binary name, which we dont need for the proejct. We will ignore it.
        args.next();

        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string.")
        }; //String needed to be searched
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path.")
        }; //Input Text File

        //Environmaent Varibale to check for the inputs' case.

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Reading a file
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|line| line.contains(&query))
    .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str, contents: &'a str) -> Vec<&'a str> {

        contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents));
    }

}