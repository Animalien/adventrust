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
    let mut total_num_output_chars1 = 0usize;
    let mut total_num_output_chars2 = 0usize;
    for line in lines {
        let mut num_input_chars = 2usize;  // for initial and final "
        let mut num_output_chars1 = 0usize;
        let mut char_iter1 = line[1..line.len()-1].chars();

        while let Some(ch1) = char_iter1.next() {
            if ch1 == '\\' {
                match char_iter1.next().unwrap() {
                    '\\' | '\"' => {
                        num_input_chars += 2;
                        num_output_chars1 += 1;
                    },
                    'x' => {
                        char_iter1.next().unwrap();
                        char_iter1.next().unwrap();
                        num_input_chars += 4;
                        num_output_chars1 += 1;
                    }
                    _ => panic!("unexpected escape sequence!")
                }
            } else {
                num_input_chars += 1;
                num_output_chars1 += 1;
            }
        }

        let mut num_output_chars2 = 2usize;  // for initial and final "
        let mut char_iter2 = line.chars();
        let mut new_st = "\"".to_string();
        while let Some(ch) = char_iter2.next() {
            match ch {
                '\"' => {
                    num_output_chars2 += 2;
                    new_st += "\\\"";
                },
                '\\' => {
                    num_output_chars2 += 2;
                    new_st += "\\\\";
                },
                _ => {
                    num_output_chars2 += 1;
                    new_st.push(ch);
                }
            }
        }
        new_st.push('\"');

        if verbose {
            println!("    For line {line}, num input chars = {num_input_chars}, num output chars 1 = {num_output_chars1}, num output chars 2 = {num_output_chars2}, new st = {new_st}");
        }

        total_num_input_chars += num_input_chars;
        total_num_output_chars1 += num_output_chars1;
        total_num_output_chars2 += num_output_chars2;
    }

    println!("  Total number of input chars = {total_num_input_chars}; total num output chars 1 = {total_num_output_chars1}; total diff 1 = {}; total num output chars 2 = {total_num_output_chars2}; total diff 2 = {}",
        total_num_input_chars - total_num_output_chars1, total_num_output_chars2 - total_num_input_chars);
}

fn main() {
    process("Examples.txt", true);
    process("Input.txt", false);
}
