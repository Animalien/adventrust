use md5::{Md5, Digest};

use adventlib::*;


const ENABLE_VERBOSE: bool = false;

fn result_has_zero_prefix(result: &[u8], num_zeroes: usize) -> bool {
    let num_bytes = num_zeroes >> 1;

    for b in &result[0..num_bytes] {
        if *b != 0 {
            return false;
        }
    }

    if num_zeroes & 1 != 0 {
        result[num_bytes] <= 15u8
    } else {
        true
    }
}

fn process(filename: &str, num_zeroes: usize) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}, looking for {num_zeroes} leading zeroes", filename);

    for line in lines {

        let mut number = 1;

        loop {
            let string_to_hash = format!("{line}{number}");

            let mut hasher = Md5::new();
            hasher.update(string_to_hash.as_bytes());
            let result = hasher.finalize();

            if result_has_zero_prefix(&result, num_zeroes) {
                println!("  For line {line}, hashed {string_to_hash}, found: {:02X?}, the desired number is therefore {number}", result);
                break;
            } else if ENABLE_VERBOSE {
                println!("    For line {line}, {string_to_hash} doesn't work!  result hash = {:02X?}", result);
            }

            number += 1;
        }
    }
}

fn main() {
    process("Examples.txt", 5);
    process("Input.txt", 5);
    process("Input.txt", 6);
}
