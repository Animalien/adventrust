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

    let mut sum_voltages = 0usize;

    for line in lines {
        let mut line_voltage = 0usize;
        for ch_tuple in line.char_indices() {
            if ch_tuple.0 < line.len() -1 {
                let largest_next_char = &line[(ch_tuple.0+1)..].chars().max().unwrap();

                let this_voltage = (ch_tuple.1.to_digit(10).unwrap() as usize) * 10usize + largest_next_char.to_digit(10).unwrap() as usize;

                if this_voltage > line_voltage {
                    line_voltage = this_voltage;
                }
            }
        }

        if verbose {
            println!("  in line {line}, largest voltage found is {line_voltage}")
        }

        sum_voltages += line_voltage;
    }

    println!("  Total sum voltages = {sum_voltages}");
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
