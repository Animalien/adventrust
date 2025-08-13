use std::fs::read_to_string;
use std::io::Result;
use std::string::String;

pub fn read_file_lines(filename: &str) -> Result<Vec<String>> {
    Ok(read_to_string(filename)?.lines().map(String::from).collect())
}
