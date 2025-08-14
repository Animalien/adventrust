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

    let mut total_wrapping_area = 0i32;
    let mut total_ribbon_length = 0i32;

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

        let wrapping_area = area1 * 2 + area2 * 2 + area3 * 2 + smallest_area;

        let ribbon_length = min(length * 2 + width * 2, min(width * 2 + height * 2, height * 2 + length * 2)) + length * width * height;

        println!("  {length}x{width}x{height} -> wrapping area = {wrapping_area}, ribbon length = {ribbon_length}");

        total_wrapping_area += wrapping_area;
        total_ribbon_length += ribbon_length;
    }

    println!("  Total wrapping area = {total_wrapping_area}, Total ribbon length = {total_ribbon_length}");
}

fn main() {
    process("Examples.txt");
    process("Input.txt");
}
