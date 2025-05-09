use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = ArgsConfig::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Error: {e}");
        process::exit(1);
    }
}

fn run(config: ArgsConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Contents of file:\n{}", contents);

    Ok(())
}

struct ArgsConfig {
    query: String,
    filename: String
}

impl ArgsConfig {
    fn new(args: &[String]) -> Result<ArgsConfig, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(ArgsConfig { query, filename })
    }
}
