use std::fs::read_to_string;
use std::io::Result;
use std::string::String;

pub fn read_file_lines(filename: &str) -> Result<Vec<String>> {
    Ok(read_to_string(filename)?.lines().map(String::from).collect())
}

pub fn maxed_string(in_string: &str, max_len: usize) -> String {
    let mut out_string: String;
    if in_string.len() > max_len {
        out_string = (&in_string[..max_len]).to_string();
        out_string += "...";
    } else {
        out_string = in_string.to_string();
    }

    out_string
}

