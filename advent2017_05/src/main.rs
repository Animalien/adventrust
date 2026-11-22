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
                println!("  instruction got incremented from {} to {}", *curr_instruction, *curr_instruction + 1);
            }

            *curr_instruction += 1;

            num_steps += 1;
        } else {
            break;
        }
    }

    println!("  Total number of steps taken until escape = {num_steps}");
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
