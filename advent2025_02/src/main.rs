use std::fs::read_to_string;

fn id_contains_repeats(id: &str, num_repeats: usize) -> bool {
    let id_len = id.len();
    if id_len.is_multiple_of(num_repeats) {
        let chunk_id_len = id_len / num_repeats;
        for next_chunk_index in 1..num_repeats {
            if &id[0..chunk_id_len] != &id[(next_chunk_index * chunk_id_len)..((next_chunk_index + 1) * chunk_id_len)] {
                return false;
            }            
        }

        true
    } else {
        false
    }
}

fn is_id_valid(id: &str, new_rules: bool) -> bool {
    if new_rules {
        for num_repeats in 2..=id.len() {
            if id_contains_repeats(id, num_repeats) {
                return false;
            }
        }

        true
    } else {
        !id_contains_repeats(id, 2)
    }
}

fn process(filename: &str, new_rules: bool, verbose: bool) {
    println!("Processing contents of {}, {} new rules", filename, if new_rules { "using" } else { "NOT USING" });

    let contents = read_to_string(filename).unwrap();
    let ranges = contents.split(&[',', '\r', '\n'][..]).filter(|s| !s.is_empty());

    let mut invalid_id_sum = 0usize;

    for range in ranges {
        let mut endpoints = range.split('-');

        let first = endpoints.next().unwrap().parse::<usize>().unwrap();
        let second = endpoints.next().unwrap().parse::<usize>().unwrap();

        for id in first..=second {
            let id_string = id.to_string();
            if is_id_valid(&id_string, new_rules) {
                if verbose {
                    println!("  {id_string} is valid")
                }
            } else {
                if verbose {
                    println!("  {id_string} is INVALID, so invalid_id_sum = {invalid_id_sum} + {id_string} = {}", invalid_id_sum + id);
                }

                invalid_id_sum += id;
            }
        }
    }

    println!("  Invalid ID sum = {invalid_id_sum}");
}

fn main() {
    process("Example.txt", false, false);
    process("Input.txt", false, false);
    process("Example.txt", true, false);
    process("Input.txt", true, false);
}
