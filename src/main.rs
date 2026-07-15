use std::{env, error::Error, fs, process};

use rq::{grep_lines, read_file_content};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    if let Err(e) = run(&config.file_path) {
        println!("Application error : {}", e);
        process::exit(1);
    }

    let file_content = read_file_content(&config.file_path);

    if config.functionality.eq("grep") {
        println!(
            "{} {:?}",
            &config.functionality,
            grep_lines(&config.pattern, &file_content)
        )
    } else {
        println!("{}", file_content);
    }
}

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    fs::read_to_string(file_path)?;
    Ok(())
}

struct Config {
    functionality: String,
    pattern: String,
    file_path: String,
}

impl Config {
    // TODO - bug here - no support for variable argument length. Use clap
    pub fn new(args: &Vec<String>) -> Self {
        Self {
            functionality: args[2].to_string(),
            pattern: args[3].to_string(),
            file_path: args[4].to_string(),
        }
    }
}
