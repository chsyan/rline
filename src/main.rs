use std::{env, fs};
use rand::Rng;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let output_filename = &args[2];
    let input_file = fs::read_to_string(input_filename).unwrap();
    let mut input_lines: Vec<&str> = input_file.lines().collect();
    let mut output_lines: Vec<&str> = Vec::with_capacity(input_lines.len());
    while input_lines.len() > 0 {
        let line = &input_lines.remove(rand::thread_rng().gen_range(0..input_lines.len()));
        output_lines.push(line);
        println!("{}", line);
    }
    let mut output_file = output_lines.join("\n");
    output_file.push_str("\n");
    fs::write(output_filename, output_file)?;
    Ok(())
}
