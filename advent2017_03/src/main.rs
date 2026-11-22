use std::collections::{HashMap, HashSet};

fn process(cases: &HashSet<isize>, test_value: isize, verbose: bool) {
    let max = cases.iter().max().unwrap();

    let mut x = 0isize;
    let mut y = 0isize;
    let mut number = 1isize;
    let mut square_size = 1isize;
    let mut which_side = 0isize;
    let mut which_number_on_side = 0isize;

    let step = HashMap::<isize, (isize, isize)>::from([
        (0, (0, 1)),
        (1, (-1, 0)),
        (2, (0, -1)),
        (3, (1, 0)),
    ]);

    let mut values = HashMap::<(isize, isize), isize>::new();

    let mut found_value_above_test = false;
    while !found_value_above_test || number <= *max {
        if cases.contains(&number) || verbose {
            println!("{number} is at coordinates ({x}, {y}), which is {} steps", x.abs() + y.abs());

            if verbose {
                println!("  square_size = {square_size}, which_side = {which_side}, which_number_on_side = {which_number_on_side}");
            }
        }

        if number == 1 {
            values.insert((0, 0), 1);
            if verbose {
                println!("number {number}, now at 0, 0");
            }
        } else if !found_value_above_test {
            let mut new_value = 0;
            if verbose {
                println!("number {number}, now at {x}, {y}");
            }
            for xx in (x-1)..=(x+1) {
                for yy in (y-1)..=(y+1) {
                    if xx == x && yy == y {
                        continue;
                    }

                    if let Some(adjacent_value) = values.get(&(xx, yy)) {
                        if verbose {
                            print!("  ({xx},{yy}): nv = {new_value} + {adjacent_value}");
                        }
                        new_value += adjacent_value;
                        if verbose {
                            println!(" = {new_value}");
                        }
                    } else {
                        if verbose {
                            println!("  {xx}, {yy}, no value yet");
                        }
                    }
                }
            }
            values.insert((x, y), new_value.clone());

            if !found_value_above_test && new_value > test_value {
                println!("  *** First new value above {test_value} = {new_value}");
                found_value_above_test = true;
            } else {
                if verbose {
                    println!("  new_value {new_value}");
                }
            }

            assert!(new_value != 0);
        }

        number += 1;
        if which_number_on_side >= square_size - 1 {
            which_side += 1;
            which_number_on_side = 1;
            if number == 2 || which_side >= 4 {
                x += 1;
                which_side = 0;
                square_size += 2;
            } else {
                let this_step = step.get(&which_side).unwrap();
                x += this_step.0;
                y += this_step.1;
            }
        } else {
            which_number_on_side += 1;
            let this_step = step.get(&which_side).unwrap();
            x += this_step.0;
            y += this_step.1;
        }
    }

}

fn main() {
    let mut cases = HashSet::<isize>::new();
    cases.insert(1);
    cases.insert(12);
    cases.insert(23);
    cases.insert(1024);
    cases.insert(325489);
    process(&cases, 325489, false);
}
