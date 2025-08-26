use md5::{Md5, Digest};

use adventlib::*;

fn result_has_five_zero_prefix(result: &[u8]) -> bool {
    if result.len() < 3 {
        return false;
    }

    result[0] == 0 && result[1] == 0 && result[2] <= 15
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

        let mut number = 1;

        loop {
            let string_to_hash = format!("{line}{number}");

            let mut hasher = Md5::new();
            hasher.update(string_to_hash.as_bytes());
            let result = hasher.finalize();

            if result_has_five_zero_prefix(&result) {
                println!("  For line {line}, hashed {string_to_hash}, found: {:02X}{:02X}{:02X}", result[0], result[1], result[2]);
                break;
            }
            //else {
            //    println!("    For line {line}, {string_to_hash} doesn't work!  result hash = {:02X?}", result);
            //}

            number += 1;
        }
    }
}

fn main() {
    process("Examples.txt");
    process("Input.txt");
}
