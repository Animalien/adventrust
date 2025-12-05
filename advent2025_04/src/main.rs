use adventlib::*;

fn num_rolls_at_xy(grid: &Vec<String>, x_center: usize, y_center: usize) -> usize {
    let mut num_rolls = 0usize;
    let y0 = (y_center as isize - 1).max(0) as usize;
    let y1 = (y_center + 1).min(grid.len() - 1);
    for y in y0..=y1 {
        let grid_line = &grid[y];
        let x0 = (x_center as isize - 1).max(0) as usize;
        let x1 = (x_center + 1).min(grid_line.len() - 1);
        num_rolls += grid_line.chars().skip(x0).take(x1 - x0 + 1).filter(|ch| *ch == '@').count();
    }

    num_rolls
}

fn process(filename: &str, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    let mut num_accessible_rolls = 0usize;

    let x_grid_size = lines[0].len();
    let y_grid_size = lines.len();

    for y in 0..y_grid_size {
        for x in 0..x_grid_size {
            if lines[y].chars().nth(x).unwrap() == '@' {
                let num_adjacent_rolls = num_rolls_at_xy(&lines, x, y) - 1;
                if num_adjacent_rolls < 4 {
                    num_accessible_rolls += 1;

                    if verbose {
                        println!("  at {x},{y}, found {num_adjacent_rolls} adjacent rolls, making this one accessible");
                    }
                }
            }
        }
    }

    println!("  Total num accessible rolls = {num_accessible_rolls}");
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
