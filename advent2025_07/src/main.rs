use adventlib::*;

fn get_grid_char(grid: &Vec<String>, column: usize, row: usize) -> char {
    grid[row].as_bytes()[column] as char
}

fn set_grid_char(grid: &mut Vec<String>, column: usize, row: usize, new_char: char) {
    unsafe {
        grid[row].as_bytes_mut()[column] = new_char as u8;
    }
}

fn iterate_grid(grid: &mut Vec<String>, num_splits: &mut usize) -> bool {
    let start_column = grid[0].find('S').unwrap();
    if get_grid_char(grid, start_column, 1) == '.' {
        // just started
        set_grid_char(grid, start_column, 1, '|');
        return false;
    }

    let mut new_grid = grid.clone();
    let mut reached_bottom = false;
    for row in 1..(grid.len()-1) {
        for column in 0..grid[0].len() {
            if get_grid_char(grid, column, row) == '|' {
                match get_grid_char(grid, column, row + 1) {
                    '.' => {
                        set_grid_char(&mut new_grid, column, row + 1, '|');
                        if row >= grid.len() - 2 {
                            reached_bottom = true;
                        }
                    }
                    '^' => {
                        let mut splitted = false;
                        if column > 0 && get_grid_char(grid, column - 1, row + 1) == '.' {
                            set_grid_char(&mut new_grid, column - 1, row + 1, '|');
                            splitted = true;
                        }
                        if column < (grid[0].len() - 1) && get_grid_char(grid, column + 1, row + 1) == '.' {
                            set_grid_char(&mut new_grid, column + 1, row + 1, '|');
                            splitted = true;
                        }
                        if splitted {
                            *num_splits += 1;

                            if row >= grid.len() - 2 {
                                reached_bottom = true;
                            }
                        }
                    }
                    _ => ()
                }
            }
        }
    }

    *grid = new_grid;

    reached_bottom
}

fn count_grid_universes(grid: &Vec<String>) -> usize {
    let mut grid_info = Vec::<Vec<usize>>::new();

    let grid_width = grid[0].len();
    grid_info.push((0..grid_width).map(|_| 0).collect::<Vec<usize>>());

    for row_index in 1..grid.len() {
        let mut grid_info_row = (0..grid_width).map(|_| 0).collect::<Vec<usize>>();

        for column_index in 0..grid_width {
            if get_grid_char(grid, column_index, row_index) == '|' {
                let above_char = get_grid_char(grid, column_index, row_index - 1);
                if above_char == 'S' {
                    grid_info_row[column_index] = 1;
                } else if above_char == '|' {
                    grid_info_row[column_index] = grid_info[row_index - 1][column_index];
                }

                if (column_index > 0) && get_grid_char(grid, column_index - 1, row_index) == '^' {
                    grid_info_row[column_index] += grid_info[row_index - 1][column_index - 1];
                }
                
                if (column_index < (grid_width - 1)) && get_grid_char(grid, column_index + 1, row_index) == '^' {
                    grid_info_row[column_index] += grid_info[row_index - 1][column_index + 1];
                }
            }
        }

        grid_info.push(grid_info_row);
    }

    let mut num_universes = 0usize;
    for grid_info_bottom_cell in grid_info.last().unwrap() {
        num_universes += grid_info_bottom_cell;
    }

    num_universes
}

fn process(filename: &str, verbose: bool) {
    let mut grid = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {filename}");

    let mut num_splits = 0usize;
    while !iterate_grid(&mut grid, &mut num_splits) {
        if verbose {
            println!("    New iteration:");
            for grid_line in &grid {
                println!("      {grid_line}");
            }
        }
    }

    if verbose {
        println!("    Final iteration:");
        for grid_line in &grid {
            println!("      {grid_line}");
        }
    }

    println!("  Num total splits = {num_splits}");

    println!("  Num universes = {}", count_grid_universes(&grid));
}

fn main() {
    process("Example.txt", true);
    process("Input.txt", false);
}
