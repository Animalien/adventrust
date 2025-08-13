use std::string::String;

use adventlib::*;

fn process_line(line: &str) -> i32 {
    let mut floor = 0;

    for c in line.chars() {
        match c {
            '(' => {
                floor = floor + 1;
            }
            ')' => {
                floor = floor - 1;
            }
            _ => panic!("Malformed line: {}", line)
        }
    }

    floor
}

fn process(filename: &str) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);
    for line in lines {
        let floor = process_line(&line);
        let line_to_print = if line.len() > 20 {
            &(String::from(&line[..20]) + "...")
        } else {
            &line
        };

        println!("  Line '{line_to_print}' takes us to {floor}");
    }
}

fn main() {
    process("Examples.txt");
    process("Input.txt");
}
