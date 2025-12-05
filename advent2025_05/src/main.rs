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

    let mut fresh_id_ranges = Vec::<(usize, usize)>::new();

    let mut lines_iter = lines.iter().peekable();

    while !lines_iter.peek().unwrap().is_empty() {
        let range_line = lines_iter.next().unwrap();
        let mut range_iter = range_line.split('-');
        let range_start = range_iter.next().unwrap().parse::<usize>().unwrap();
        let range_end = range_iter.next().unwrap().parse::<usize>().unwrap();

        fresh_id_ranges.push((range_start, range_end));
    }
    lines_iter.next();

    let mut num_fresh_ingredients = 0usize;
    while lines_iter.peek().is_some() {
        let line = lines_iter.next().unwrap();
        let id = line.parse::<usize>().unwrap();

        let mut is_fresh = false;
        for range in &fresh_id_ranges {
            if (id >= range.0) && (id <= range.1) {
                is_fresh = true;

                if verbose {
                    println!("  {id} is fresh because it is in the range {}-{}", range.0, range.1);
                }

                break;
            }
        }

        if is_fresh {
            num_fresh_ingredients += 1;
        } else {
            if verbose {
                println!("  {id} is spoiled!");
            }
        }
    }

    println!("  Total number of fresh ingredients = {num_fresh_ingredients}");
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
