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

    let mut dial = 50i32;
    if verbose {
        println!("  {dial}");
    }

    let mut num_times_zero = 0u32;

    for line in lines {
        let left = line.chars().next().unwrap() == 'L';
        let delta = &line[1..].parse().unwrap();

        if left {
            dial = (dial - delta).rem_euclid(100);
        } else {
            dial = (dial + delta).rem_euclid(100);
        }

        if verbose {
            println!("  {dial}");
        }

        if dial == 0 {
            num_times_zero += 1;
        }
    }

    println!("  Total number of zero visits = {num_times_zero}");
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
