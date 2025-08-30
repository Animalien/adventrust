use adventlib::*;

const GRID_EDGE_SIZE: usize = 1000;
const GRID_TOTAL_SIZE: usize = GRID_EDGE_SIZE * GRID_EDGE_SIZE;

fn position_string_to_numbers(pos_string: &str) -> (usize, usize) {
    let mut ps_iter = pos_string.split(',');
    (ps_iter.next().unwrap().parse().unwrap(), ps_iter.next().unwrap().parse().unwrap())
}

fn process(filename: &str, new_rules: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    let mut light_grid: Vec<u32> = std::iter::repeat(0u32).take(GRID_TOTAL_SIZE).collect();
    let mut light_number = 0;

    enum LightOper {
        TurnOn,
        TurnOff,
        Toggle,
    }

    for line in lines {
        let mut token_iter = line.split(' ');

        let light_oper = match token_iter.next().unwrap() {
            "turn" => {
                match token_iter.next().unwrap() {
                    "on" => LightOper::TurnOn,
                    "off" => LightOper::TurnOff,
                    _ => panic!("Invalid second word")
                }
            },
            "toggle" => {
                LightOper::Toggle
            }
            _ => panic!("Invalid first word")
        };

        let p0 = position_string_to_numbers(token_iter.next().unwrap());

        let through = token_iter.next().unwrap();
        assert_eq!(through, "through");

        let p1 = position_string_to_numbers(token_iter.next().unwrap());

        assert!(p0.0 <= p1.0);
        assert!(p0.1 <= p1.1);

        let mut mod_func: Box<dyn FnMut(usize, usize)> = match light_oper {
            LightOper::TurnOn => {
                Box::new(|x: usize, y: usize| {
                    let entry: &mut u32 = &mut light_grid[GRID_EDGE_SIZE * y + x];
                    if *entry == 0 || new_rules {
                        *entry += 1;
                        light_number += 1;
                    }
                })
            },
            LightOper::TurnOff => {
                Box::new(|x: usize, y: usize| {
                    let entry: &mut u32 = &mut light_grid[GRID_EDGE_SIZE * y + x];
                    if *entry > 0 {
                        *entry -= 1;
                        light_number -= 1;
                    }
                })
            },
            LightOper::Toggle => {
                if new_rules {
                    Box::new(|x: usize, y: usize| {
                        let entry: &mut u32 = &mut light_grid[GRID_EDGE_SIZE * y + x];
                        *entry += 2;
                        light_number += 2;
                    })
                } else {
                    Box::new(|x: usize, y: usize| {
                    let entry: &mut u32 = &mut light_grid[GRID_EDGE_SIZE * y + x];
                        if *entry == 1 {
                            *entry = 0;
                            light_number -= 1;
                        } else {
                            *entry = 1;
                            light_number += 1;
                        }
                    })
                }
            }
        };

        for x in p0.0..=p1.0 {
            for y in p0.1..=p1.1 {
                mod_func(x, y);
            }
        }
    }

    println!("  {} = {light_number}", if new_rules { "Total light brightness" } else { "Number of lights that are on" });
}

fn main() {
    process("Examples.txt", false);
    process("Input.txt", false);
    process("Input.txt", true);
}
