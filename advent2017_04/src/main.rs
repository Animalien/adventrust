use std::collections::{BTreeMap, HashSet};

use adventlib::*;

fn process(filename: &str, do_permutations: bool, verbose: bool) {
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

        if do_permutations {
            let mut perm_sets = HashSet::<BTreeMap<char, usize>>::new();
            for word in line.split_whitespace() {
                let mut char_map = BTreeMap::<char, usize>::new();
                for ch in word.chars() {
                    *char_map.entry(ch).or_insert(0) += 1;
                }
                if !perm_sets.insert(char_map.clone()) {
                    if verbose {
                        println!("    word {word} shows up in the list as a different permutation\n    passphrase is INvalid");
                    }
                    continue 'outer;
                }
            }
        } else {
            let mut word_set = HashSet::<&str>::new();
            for word in line.split_whitespace() {
                if !word_set.insert(word) {
                    if verbose {
                        println!("    word {word} is duplicated\n    passphrase is INvalid");
                    }
                    continue 'outer;
                }
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
    process("Example1.txt", false, true);
    process("Input.txt", false, false);
    process("Example2.txt", true, true);
    process("Input.txt", true, false);
}
