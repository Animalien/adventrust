use adventlib::*;

fn process(filename: &str, part_two: bool, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    let mut instructions = lines.iter().map(|line| line.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let mut num_steps = 0usize;
    let mut instruction_index = 0isize;
    loop {
        if instruction_index >= 0 && let Some(curr_instruction) = instructions.get_mut(instruction_index as usize) {
            if verbose {
                println!("  instruction_index = {} + {} = {}", instruction_index, *curr_instruction, instruction_index + *curr_instruction);
            }

            instruction_index += *curr_instruction;

            if verbose {
                print!("  instruction got changed from {} to ", *curr_instruction);
            }

            if part_two && *curr_instruction >= 3 {
                *curr_instruction -= 1;
            } else {
                *curr_instruction += 1;
            }

            if verbose {
                println!("{}", *curr_instruction);
            }

            num_steps += 1;
        } else {
            break;
        }
    }

    println!("  Total number of steps taken until escape = {num_steps}");
}

fn main() {
    process("Example.txt", false, true);
    process("Input.txt", false, false);
    process("Example.txt", true, true);
    process("Input.txt", true, false);
}
