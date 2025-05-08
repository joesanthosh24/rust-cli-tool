use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = ArgsConfig::new(&args);

    let contents = fs::read_to_string(config.filename).expect("Error reading the file");

    println!("Contents of file:\n{}", contents);
}

struct ArgsConfig {
    query: String,
    filename: String
}

impl ArgsConfig {
    fn new(args: &[String]) -> ArgsConfig {
        let query = args[1].clone();
        let filename = args[2].clone();
    
        ArgsConfig { query, filename }
    }
}
