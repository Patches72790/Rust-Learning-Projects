use minigrep::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename);

    if let Err(err) = run(config) {
        eprintln!("There was a problem reading the file: {}", err);
        process::exit(2);
    };
}
