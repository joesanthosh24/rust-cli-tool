use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: ArgsConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive { search(&config.query, &contents) } 
        else { search_case_insensitive(&config.query, &contents) };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct ArgsConfig {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl ArgsConfig {
    pub fn new(args: &[String]) -> Result<ArgsConfig, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(ArgsConfig { query, filename, case_sensitive })
    }
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result() {
        let query = "My Name";
        let contents = "\
Hi
My Name
is Joe";
        assert_eq!(vec!["My Name"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "mY naMe";
        let contents = "\
Hi
My Name
is Joe";
        assert_eq!(
            vec!["My Name"],
            search_case_insensitive(query, contents)
        );
    }
}