use std::{env, error::Error, fs, process};

use rq::read_file_content;
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    if let Err(e) = run(&file_path) {
        println!("Application error : {}", e);
        process::exit(1);
    }

    let file_content = read_file_content(&file_path);

    println!("{}", file_content);
    
}

fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    fs::read_to_string(file_path)?;
    Ok(())
 }