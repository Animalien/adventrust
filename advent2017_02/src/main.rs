use adventlib::*;

fn process(filename: &str, do_division_sum: bool, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {filename}");

    let mut sum = 0usize;
    for line in lines {
        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        if do_division_sum {
            let mut division = Option::<usize>::default();

            for (index1, number1) in numbers.iter().enumerate() {
                for index2 in (index1+1)..numbers.len() {
                    let number2 = numbers[index2];

                    if *number1 > number2 {
                        if (*number1 % number2) == 0 {
                            division = Some(*number1 / number2);

                            if verbose {
                                print!("  {} is divisible by {}, ", *number1, number2);
                            }
                            break;
                        }
                    } else if number2 > *number1 {
                        if (number2 % *number1) == 0 {
                            division = Some(number2 / *number1);

                            if verbose {
                                print!("  {} is divisible by {}, ", number2, *number1);
                            }
                            break;
                        }
                    }
                }
            }

            sum += division.unwrap();

            if verbose {
                println!("division = {}, sum {sum}", division.unwrap());
            }
        } else {
            let mut min = Option::<usize>::default();
            let mut max = Option::<usize>::default();

            for number in numbers {
                if min.is_none() || min.unwrap() > number {
                    min = Some(number);
                }
                if max.is_none() || max.unwrap() < number {
                    max = Some(number);
                }
            }

            let diff = max.unwrap() - min.unwrap();
            sum += diff;

            if verbose {
                println!("  min = {}, max = {}, checksum + {diff} = {sum}", min.unwrap(), max.unwrap());
            }
        }
    }

    println!("  {} = {sum}", if do_division_sum { "division_sum" } else { "checksum" });
}

fn main() {
    process("Example1.txt", false, true);
    process("Input.txt", false, false);
    process("Example2.txt", true, true);
    process("Input.txt", true, false);
}
