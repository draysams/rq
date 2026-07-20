use serde::{Deserialize, Serialize};
use std::fs::{self};

pub fn read_file_content(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("File should exist")
}

pub fn count_stats(content: &str) -> (usize, usize, usize) {
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();

    (lines, words, chars)
}

pub fn grep_lines<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let found_lines: &mut Vec<&str> = &mut Vec::<&str>::new();
    for line in content.lines() {
        if line.contains(pattern) {
            found_lines.push(line);
        }
    }
    found_lines.to_vec()
}

pub fn read_people<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
) -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    for result in reader.deserialize::<Person>() {
        let person = result?;
        results.push(person);
    }
    Ok(results)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub city: String,
}

#[cfg(test)]
mod tests {
    use crate::count_stats;

    #[test]
    fn counts_a_simple_string() {
        assert_eq!(count_stats("Hello\nworld"), (2, 2, 11));
    }

    #[test]
    fn empty_string_is_all_zero() {
        assert_eq!(count_stats(""), (0, 0, 0));
    }
}

#[cfg(test)]
mod grep_tests {
    use super::*;

    #[test]
    fn find_matching_lines() {
        let text = "banana\napple\ngrapefruit";
        assert_eq!(grep_lines("apple", text), vec!["apple"])
    }

    #[test]
    fn no_match_returns_empty() {
        let text = "apple\nbanana";
        assert_eq!(grep_lines("orange", text), Vec::<&str>::new())
    }

    #[test]
    fn case_sensitive_by_default() {
        let text = "Apple\napple";
        assert_eq!(grep_lines("apple", text), vec!["apple"]);
    }
}
