use std::{env, error::Error, fs};
use std::io::Write;

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub message: String,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("not enough arguments".into());
        }

        let file_path = args[1].clone();
        let message = args[2].clone();

        Ok(Config { file_path, message })
    }
}

pub fn get_an_appending_file_handler(file_path: &str) -> Result<fs::File, Box<dyn Error>> {
    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    Ok(file)
}

pub fn write_to_file(file: &mut fs::File, message: &str,new_line:bool) -> Result<(), Box<dyn Error>> {
    if new_line {
        writeln!(file, "{}", message)?;
    } else {
        write!(file, "{}", message)?;
    }
    Ok(())
}