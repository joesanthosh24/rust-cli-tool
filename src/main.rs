use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _query:  &String = &args[1];
    let filename: &String = &args[2];

    let contents = fs::read_to_string(filename).expect("Error reading the file");

    println!("Contents of file:\n{}", contents);
}
