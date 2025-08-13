use std::string::String;

use adventlib::*;

fn process_line(line: &str) -> (i32, Option<usize>) {
    let mut floor: i32 = 0;
    let mut entered_basement_index: Option<usize> = None;

    let mut char_index: usize = 1;

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

        if entered_basement_index == None && floor < 0 {
            entered_basement_index = Some(char_index);
        }

        char_index = char_index + 1;
    }

    (floor, entered_basement_index)
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
        let (floor, entered_basement_index) = process_line(&line);
        let line_to_print = if line.len() > 20 {
            &(String::from(&line[..20]) + "...")
        } else {
            &line
        };

        println!("  Line '{line_to_print}' takes us to {floor}, and basement was entered at char index {}",
            match entered_basement_index {
                Some(value) => {
                    value.to_string()
                }
                _ => {
                    "None".to_string()
                }
            }
        );
    }
}

fn main() {
    process("Examples.txt");
    process("Input.txt");
}
