use std::collections::{HashMap, HashSet};

use adventlib::*;

fn is_string_nice(full_vowel_set: &HashSet<char>, in_string: &str, new_rules: bool) -> bool {
    if new_rules {
        is_string_nice_new_rules(in_string)
    } else {
        is_string_nice_old_rules(full_vowel_set, in_string)
    }
}

fn is_string_nice_old_rules(full_vowel_set: &HashSet<char>, in_string: &str) -> bool {
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

fn is_string_nice_new_rules(in_string: &str) -> bool {
    let mut char_iter = in_string.chars();
    let mut prev_prev_char = char_iter.next().unwrap();
    let mut prev_char = char_iter.next().unwrap();

    let mut two_seq_map = HashMap::from([((prev_prev_char, prev_char), 1)]);

    let mut found_xyx = false;
    let mut found_mult_2seq = false;
    let mut inserted_seq2_prev = true;

    while let Some(curr_char) = char_iter.next() {
        if curr_char == prev_prev_char {
            found_xyx = true;
        }

        if !inserted_seq2_prev || prev_prev_char != prev_char || prev_char != curr_char || prev_prev_char != curr_char {
            let entry = two_seq_map.entry((prev_char, curr_char)).or_insert(0);
            *entry += 1;
            inserted_seq2_prev = true;

            if *entry > 1 {
                found_mult_2seq = true;
            }
        } else {
            inserted_seq2_prev = false;
        }

        prev_prev_char = prev_char;
        prev_char = curr_char;
    }

    found_xyx && found_mult_2seq
}

fn process(filename: &str, new_rules: bool, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}, {}", filename, if new_rules { "with new rules" } else { "with normal rules" });

    let full_vowel_set = HashSet::from(['a', 'e', 'i', 'o', 'u']);

    let mut num_nice: usize = 0;
    for line in lines {
        if is_string_nice(&full_vowel_set, &line, new_rules) {
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

    println!("  Number of nice strings = {num_nice}");
}

fn main() {
    process("Examples.txt", false, true);
    process("Input.txt", false, false);
    process("Examples2.txt", true, true);
    process("Input.txt", true, false);
}
