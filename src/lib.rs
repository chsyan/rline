use std::{fs, error, env};
use rand::Rng;

pub struct Config {
    input_file: String,
}


impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let input_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Input file is required"),
        };
        Ok(Config { input_file })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let in_str = fs::read_to_string(config.input_file).unwrap();
    let mut in_lines: Vec<&str> = in_str.lines().collect();
    while in_lines.len() > 0 {
        println!("{}", &in_lines.remove(rand::thread_rng().gen_range(0..in_lines.len())));
    }
    Ok(())
}


