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

    let mut total_num_input_chars = 0usize;
    let mut total_num_output_chars = 0usize;
    for line in lines {
        let mut num_input_chars = 2usize;  // for initial and final "
        let mut num_output_chars = 0usize;
        let mut char_iter = line[1..line.len()-1].chars();

        while let Some(ch1) = char_iter.next() {
            if ch1 == '\\' {
                match char_iter.next().unwrap() {
                    '\\' | '\"' => {
                        num_input_chars += 2;
                        num_output_chars += 1;
                    },
                    'x' => {
                        char_iter.next().unwrap();
                        char_iter.next().unwrap();
                        num_input_chars += 4;
                        num_output_chars += 1;
                    }
                    _ => panic!("unexpected escape sequence!")
                }

            } else {
                num_input_chars += 1;
                num_output_chars += 1;
            }
        }

        if verbose {
            println!("    For line {line}, num input chars = {num_input_chars}, num output chars = {num_output_chars}");
        }

        total_num_input_chars += num_input_chars;
        total_num_output_chars += num_output_chars;
    }

    println!("  Total number of input chars = {total_num_input_chars}; total num output chars = {total_num_output_chars}; total diff = {}", total_num_input_chars - total_num_output_chars);
}

fn main() {
    process("Examples.txt", true);
    process("Input.txt", false);
}
