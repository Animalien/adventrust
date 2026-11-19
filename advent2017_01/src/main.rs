use adventlib::*;

fn process(filename: &str, half_way_around: bool, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {filename}, half_way_around = {half_way_around}");

    for line in lines {
        if verbose {
            println!("Line = {line}:");
        }

        let mut total = 0usize;
        let line_bytes = line.as_bytes();

        let num_bytes = line_bytes.len();
        let stepper = if half_way_around { num_bytes / 2 } else { 1 };
        for byte_index in 0..num_bytes {
            let curr_byte = line_bytes[byte_index] - ('0' as u8);
            let next_byte_index = (byte_index + stepper) % num_bytes;
            let next_byte = line_bytes[next_byte_index] - ('0' as u8);
            if curr_byte == next_byte {
                total += curr_byte as usize;

                if verbose {
                    println!("  {curr_byte} (at index {byte_index}) == {next_byte} ({stepper} characters ahead at index {next_byte_index}), so total is now {total}");
                }
            } else {
                if verbose {
                    println!("  {curr_byte} (at index {byte_index}) != {next_byte} ({stepper} characters ahead at index {next_byte_index})");
                }
            }
        }

        if verbose {
            print!("  ");
        }

        println!("Total = {total}");
    }
}

fn main() {
    process("Examples1.txt", false, true);
    process("Input.txt", false, false);
    process("Examples2.txt", true, true);
    process("Input.txt", true, false);
}
