use adventlib::*;

fn process(filename: &str) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("  Contents:");
    for line in lines {
        println!("    {}", line);
    }
}

fn main() {
    process("BeaverFuck.txt");
    process("Examples.txt");
    process("Input.txt");
}
