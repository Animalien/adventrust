use adventlib::*;

fn process(filename: &str, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {filename}");

    let mut checksum = 0usize;
    for line in lines {
        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let mut min = Option::<usize>::default();
        let mut max = Option::<usize>::default();

        for number in numbers {
            if min.is_none() || min.unwrap() > number {
                min = Some(number);
            }
            if max.is_none() || max.unwrap() < number {
                max = Some(number);
            }
        }

        let diff = max.unwrap() - min.unwrap();
        checksum += diff;

        if verbose {
            println!("  min = {}, max = {}, checksum + {diff} = {checksum}", min.unwrap(), max.unwrap());
        }
    }

    println!("  Checksum = {checksum}");
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
