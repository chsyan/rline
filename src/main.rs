use std::{env, fs, process, error};
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Runtime error: {}", err);
        process::exit(1);

    }
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let in_str = fs::read_to_string(config.in_file).unwrap();
    let mut in_lines: Vec<&str> = in_str.lines().collect();
    let mut out_lines: Vec<&str> = Vec::with_capacity(in_lines.len());
    while in_lines.len() > 0 {
        let line = &in_lines.remove(rand::thread_rng().gen_range(0..in_lines.len()));
        out_lines.push(line);
        println!("{}", line);
    }
    let mut out_str = out_lines.join("\n");
    out_str.push_str("\n");
    fs::write(config.out_file, out_str)?;
    Ok(())
}


struct Config {
    in_file: String,
    out_file: String,
}


impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let in_file = args[1].clone();
        let out_file = args[2].clone();

        Ok(Config { in_file, out_file })
    }
}
