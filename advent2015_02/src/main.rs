use std::cmp::min;

use adventlib::*;

fn process(filename: &str) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    let mut grand_total = 0i32;

    println!("Processing contents of {}", filename);
    for line in lines {
        let mut token_iter = line.split('x');
        let length: i32 = token_iter.next().unwrap().parse().unwrap();
        let width: i32 = token_iter.next().unwrap().parse().unwrap();
        let height: i32 = token_iter.next().unwrap().parse().unwrap();

        let area1 = length * width;
        let area2 = width * height;
        let area3 = height * length;

        let smallest_area = min(area1, min(area2, area3));

        let line_total = area1 * 2 + area2 * 2 + area3 * 2 + smallest_area;
        println!("  {length}x{width}x{height} -> {line_total}");

        grand_total += line_total;
    }

    println!("  Grand total = {grand_total}");
}

fn main() {
    process("Examples.txt");
    process("Input.txt");
}
