use adventlib::*;

fn process(filename: &str) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    let mut total_distance = 0usize;
    let mut largest_distance: Option<usize> = None;

    for line in lines {
        let this_distance = line.rsplit(' ').next().unwrap().parse::<usize>().unwrap();
        match largest_distance {
            None => largest_distance = Some(this_distance),
            Some(largest_distance_value) => {
                if this_distance > largest_distance_value {
                    largest_distance = Some(this_distance)
                }
            }
        }
        total_distance += this_distance;
    }

    println!("  Total distance = {total_distance}, largest distance = {:?}, so distance of shortest route = {}", largest_distance, total_distance - largest_distance.unwrap());
}

fn main() {
    process("Example.txt");
    process("Input.txt");
}
