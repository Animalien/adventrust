use std::collections::HashSet;

use adventlib::*;

fn is_string_nice(full_vowel_set: &HashSet<char>, in_string: &str) -> bool {
    let mut char_iter = in_string.chars();
    let mut prev_char = char_iter.next().unwrap();

    let mut num_vowels = if full_vowel_set.contains(&prev_char) { 1 } else { 0 };
    let mut found_repeat = false;

    while let Some(curr_char) = char_iter.next() {
        if full_vowel_set.contains(&curr_char) {
            num_vowels += 1;
        }

        if curr_char == prev_char {
            found_repeat = true;
        } else if (prev_char == 'a' && curr_char == 'b') ||
                    (prev_char == 'c' && curr_char == 'd') ||
                    (prev_char == 'p' && curr_char == 'q') ||
                    (prev_char == 'x' && curr_char == 'y') {
            return false;
        }

        prev_char = curr_char;
    }

    num_vowels >= 3 && found_repeat
}

fn process(filename: &str, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    let full_vowel_set = HashSet::from(['a', 'e', 'i', 'o', 'u']);

    let mut num_nice: usize = 0;
    for line in lines {
        if is_string_nice(&full_vowel_set, &line) {
            num_nice += 1;
            if verbose {
                println!("  {line} is NICE!");
            }
        } else {
            if verbose {
                println!("  {line} is naughty");
            }
        }
    }

    println!("  Number of nice strings = {num_nice}")
}

fn main() {
    process("Examples.txt", true);
    process("Input.txt", false);
}
