use std::fs;

pub fn read_file_content(file_path: &str) -> String {
    return fs::read_to_string(&file_path).expect("File should exist");
}