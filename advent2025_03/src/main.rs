use adventlib::*;

fn voltage_from_string(string: &str, num_chars: usize) -> usize {
    if string == "811111111111119" && num_chars == 12 {
        println!("THE PROBLEMO")
    }

    let mut next_char_index = 0usize;
    let mut num_chars_remaining = num_chars;
    let mut voltage = 0usize;

    while num_chars_remaining > 0 {
        let num_chars_to_check = string.len() - next_char_index - num_chars_remaining + 1;
        let mut char_indices_iter = string.char_indices().skip(next_char_index).take(num_chars_to_check).peekable();
        if char_indices_iter.peek().is_none() {
            println!("    WE GOT NUN!  next_char_index = {next_char_index}, num_chars_remaining = {num_chars_remaining}, num_chars_to_check = {num_chars_to_check}, string = {string}, voltage = {voltage}");
        }
        assert!(!char_indices_iter.peek().is_none());
        let best_next_char_tuple = char_indices_iter.max_by(|lhs, rhs| if lhs.1 == rhs.1 { rhs.0.cmp(&lhs.0) } else { lhs.1.cmp(&rhs.1) }).unwrap();
        let best_next_char = best_next_char_tuple.1;
        next_char_index = best_next_char_tuple.0 + 1;
        num_chars_remaining -= 1;

        voltage = voltage * 10usize + best_next_char.to_digit(10).unwrap() as usize;
    }

    voltage
}

fn process(filename: &str, num_chars: usize, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}, using {num_chars} chars", filename);

    let mut sum_voltages = 0usize;

    for line in lines {
        let line_voltage = voltage_from_string(&line, num_chars);

        if verbose {
            println!("  in line {line}, largest voltage found is {line_voltage}")
        }

        sum_voltages += line_voltage;
    }

    println!("  Total sum voltages = {sum_voltages}");
}

fn main() {
    process("Example.txt", 2, true);
    process("Input.txt", 2, false);
    process("Example.txt", 12, true);
    process("Input.txt", 12, false);
}
