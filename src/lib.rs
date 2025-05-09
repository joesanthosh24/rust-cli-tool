use std::error::Error;
use std::fs;

pub fn run(config: ArgsConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Contents of file:\n{}", contents);

    Ok(())
}

pub struct ArgsConfig {
    pub query: String,
    pub filename: String
}

impl ArgsConfig {
    pub fn new(args: &[String]) -> Result<ArgsConfig, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(ArgsConfig { query, filename })
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
}