use adventlib::*;

const NUM_DIAL_POINTS: i32 = 100;

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

    let mut num_times_zero = 0i32;

    for line in lines {
        let prev_dial = dial;

        let left = line.chars().next().unwrap() == 'L';
        let delta: i32 = line[1..].parse().unwrap();

        let num_times_zero_this_line;
        if left {
            // flip dial
            dial = (NUM_DIAL_POINTS - dial) % NUM_DIAL_POINTS;
        }

        num_times_zero_this_line = (dial + delta) / NUM_DIAL_POINTS;
        dial = (dial + delta) % NUM_DIAL_POINTS;

        if left {
            // flip dial back again
            dial = (NUM_DIAL_POINTS - dial) % NUM_DIAL_POINTS;
        }

        num_times_zero += num_times_zero_this_line;

        if verbose {
            println!("  {prev_dial} -> {} x {delta} -> {dial};  saw zero {num_times_zero_this_line} times", if left { "left" } else { "right" });
        }
    }

    println!("  Total number of zero visits = {num_times_zero}");
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
