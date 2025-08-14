use std::collections::HashSet;

use adventlib::*;

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
        let mut x = 0i32;
        let mut y = 0i32;

        let mut house_set = HashSet::<(i32, i32)>::new();
        house_set.insert((0,0));

        for c in line.chars() {
            match c {
                '^' => {
                    y += 1;
                }
                '<' => {
                    x -= 1;
                }
                '>' => {
                    x += 1;
                }
                'v' => {
                    y -= 1;
                }
                _ => {
                    panic!("Invalid direction character in data:  {c}");
                }
            }

            house_set.insert((x, y));
        }

        println!("  In line '{line}', delivered presents to {} houses", house_set.len());
    }
}

fn main() {
    process("Examples.txt");
    process("Input.txt");
}
