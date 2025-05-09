use std::env;
use std::process;

use my_cli_tool::ArgsConfig;
use my_cli_tool::run;

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
