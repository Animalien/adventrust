use std::collections::HashSet;

use adventlib::*;

fn process(filename: &str) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    for line in lines {
        let mut x = 0i32;
        let mut y = 0i32;

        let mut house_set = HashSet::<(i32, i32)>::new();
        house_set.insert((0,0));

        for c in line.chars() {
            match c {
                '^' => {
                    y += 1;
                }
                '<' => {
                    x -= 1;
                }
                '>' => {
                    x += 1;
                }
                'v' => {
                    y -= 1;
                }
                _ => {
                    panic!("Invalid direction character in data:  {c}");
                }
            }

            house_set.insert((x, y));
        }

        let mut x1 = 0i32;
        let mut y1 = 0i32;
        let mut x2 = 0i32;
        let mut y2 = 0i32;

        let mut house_set2 = HashSet::<(i32, i32)>::new();
        house_set2.insert((0,0));

        let mut robo_santa = false;

        for c in line.chars() {
            if robo_santa {
                match c {
                    '^' => {
                        y1 += 1;
                    }
                    '<' => {
                        x1 -= 1;
                    }
                    '>' => {
                        x1 += 1;
                    }
                    'v' => {
                        y1 -= 1;
                    }
                    _ => {
                        panic!("Invalid direction character in data:  {c}");
                    }
                }

                house_set2.insert((x1, y1));

                robo_santa = false;
            } else {
                match c {
                    '^' => {
                        y2 += 1;
                    }
                    '<' => {
                        x2 -= 1;
                    }
                    '>' => {
                        x2 += 1;
                    }
                    'v' => {
                        y2 -= 1;
                    }
                    _ => {
                        panic!("Invalid direction character in data:  {c}");
                    }
                }

                house_set2.insert((x2, y2));

                robo_santa = true;
            }
        }

        println!("  In line '{}', delivered presents to {} houses with just Santa, delivered presents to {} houses with Santa plus Robo Santa", maxed_string(&line, 20), house_set.len(), house_set2.len());
    }
}

fn main() {
    process("Examples.txt");
    process("Input.txt");
}
