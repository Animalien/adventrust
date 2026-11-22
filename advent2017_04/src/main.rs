use std::collections::HashSet;

use adventlib::*;

fn process(filename: &str, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    let mut num_valid_passphrases = 0usize;
    'outer: for line in lines {
        if verbose {
            println!("  line: {line}");
        }

        let mut word_set = HashSet::<&str>::new();
        for word in line.split_whitespace() {
            if !word_set.insert(word) {
                if verbose {
                    println!("    word {word} is duplicated, so passphrase is INvalid");
                }
                continue 'outer;
            }
        }
        num_valid_passphrases += 1;
        if verbose {
            println!("    passphrase is VALID!  now num_valid_passphrases = {num_valid_passphrases}");
        }
    }

    println!("  Total number valid passphrases = {num_valid_passphrases}");
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
