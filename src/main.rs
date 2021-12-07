use std::{env, process};
use rline::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = rline::run(config) {
        eprintln!("Runtime error: {}", err);
        process::exit(1);
    }
}
