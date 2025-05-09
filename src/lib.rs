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