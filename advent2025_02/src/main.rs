use std::fs::read_to_string;

fn is_id_valid(id: &str) -> bool {
    let id_len = id.len();
    if id_len.is_multiple_of(2) {
        let half_id_len = id_len / 2;
        &id[..half_id_len] != &id[half_id_len..]
    } else {
        true
    }
}

fn process(filename: &str, verbose: bool) {
    println!("Processing contents of {}", filename);

    let contents = read_to_string(filename).unwrap();
    let ranges = contents.split(&[',', '\r', '\n'][..]).filter(|s| !s.is_empty());

    let mut invalid_id_sum = 0u64;

    for range in ranges {
        let mut endpoints = range.split('-');

        let first = endpoints.next().unwrap().parse::<u64>().unwrap();
        let second = endpoints.next().unwrap().parse::<u64>().unwrap();

        for id in first..=second {
            let id_string = id.to_string();
            if is_id_valid(&id_string) {
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
    process("Example.txt", true);
    process("Input.txt", false);
}
