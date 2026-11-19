use adventlib::*;

fn process(filename: &str, verbose: bool) {
    let mut lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    for line in &mut lines {
        let mut total = 0usize;
        let mut prev_char = line.chars().next().unwrap();
        line.push(prev_char);
        for ch in line.chars().skip(1) {
            if ch == prev_char {
                total += ((ch as u8) - ('0' as u8)) as usize;
            }
            prev_char = ch;
        }

        if verbose {
            println!("Line = {line}");
        }

        println!("Total = {total}");
    }
}

fn main() {
    process("Examples.txt", true);
    process("Input.txt", false);
}
