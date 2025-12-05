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

fn query_accessible_rolls(grid: &Vec<String>, verbose: bool) -> Vec<(usize, usize)> {
    let mut accessibles = Vec::<(usize, usize)>::new();

    let x_grid_size = grid[0].len();
    let y_grid_size = grid.len();

    for y in 0..y_grid_size {
        for x in 0..x_grid_size {
            if grid[y].chars().nth(x).unwrap() == '@' {
                let num_adjacent_rolls = num_rolls_at_xy(&grid, x, y) - 1;
                if num_adjacent_rolls < 4 {
                    accessibles.push((x, y));

                    if verbose {
                        println!("  at {x},{y}, found {num_adjacent_rolls} adjacent rolls, making this one accessible");
                    }
                }
            }
        }
    }

    accessibles
}

fn remove_accessible_rolls(grid: &mut Vec<String>, accessibles: &Vec<(usize, usize)>) {
    for (x, y) in accessibles {
        // works because I am working in single-byte ASCII :)
        grid[*y].replace_range(*x..*x+1, ".");
    }
}

fn process(filename: &str, verbose: bool) {
    let mut lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    let mut accessibles = query_accessible_rolls(&lines, verbose);
    println!("  Total initial num accessible rolls = {}", accessibles.len());

    let mut total_num_removed_rolls = 0usize;
    while !accessibles.is_empty() {
        remove_accessible_rolls(&mut lines, &accessibles);

        let num_removed_rolls = accessibles.len();
        total_num_removed_rolls += num_removed_rolls;

        accessibles = query_accessible_rolls(&lines, false);

        if verbose {
            println!("Removed {num_removed_rolls} rolls:");
            for line in &lines {
                println!("  {line}");
            }
        }
    }

    println!("  Total number of removed rolls = {total_num_removed_rolls}");
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
