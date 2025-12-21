use adventlib::*;

fn calc_columns(lines: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut widths = Vec::<usize>::new();

    for line in lines {
        let mut next_column_index = 0usize;
        let mut in_space = true;
        let mut num_chars = 0usize;

        let mut chars_iter = line.chars();
        while let Some(ch) = chars_iter.next() {
            if in_space {
                if ch != ' ' {
                    in_space = false;
                    num_chars = 1usize;
                }
            } else {
                if ch != ' ' {
                    num_chars += 1;
                } else {
                    in_space = true;
                    if widths.len() <= next_column_index {
                        assert!(widths.len() == next_column_index);
                        widths.push(num_chars);
                    } else {
                        widths[next_column_index] = widths[next_column_index].max(num_chars);
                        num_chars = 0usize;
                    }
                    next_column_index += 1;
                }
            }
        }
        if !in_space {
            if widths.len() <= next_column_index {
                assert!(widths.len() == next_column_index);
                widths.push(num_chars);
            } else {
                widths[next_column_index] = widths[next_column_index].max(num_chars);
            }
        }
    }

    let mut starts = widths.clone();
    let mut position = 0usize;
    for column_index in 0..widths.len() {
        starts[column_index] = position;
        position += widths[column_index];
        position += 1;
    }

    (starts, widths)
}

fn create_matrix(lines: &Vec<String>, starts: &Vec<usize>, widths: &Vec<usize>) -> (Vec<Vec<String>>, Vec<bool>) {
    let mut matrix = Vec::<Vec::<String>>::new();
    matrix.reserve(lines.len() - 1);

    for row_index in 0..(lines.len()-1) {
        let mut matrix_row = Vec::<String>::new();
        matrix_row.reserve(starts.len());

        let line = &lines[row_index];
        for column_index in 0..starts.len() {
            let column_width = widths[column_index];
            let first_char_index = starts[column_index];
            let last_char_index = first_char_index + column_width - 1;
            matrix_row.push(format!("{:<width$}", &line[first_char_index..=last_char_index], width = column_width));
        }

        matrix.push(matrix_row);
    }

    let oper_mults = lines.last().unwrap().split_whitespace().map(|item| item == "*").collect::<Vec<bool>>();

    (matrix, oper_mults)
}

fn get_number_from_matrix(matrix: &Vec<Vec<String>>, column_index: usize, row_index: usize, new_style: bool) -> usize {
    if !new_style {
        return matrix[row_index][column_index].split_whitespace().next().unwrap().parse::<usize>().unwrap();
    }

    let mut num_string = String::new();
    for actual_row_index in 0..matrix.len() {
        let matrix_item = &matrix[actual_row_index][column_index];
        let ch = matrix_item.chars().rev().skip(row_index).next().unwrap();
        if ch != ' ' {
            num_string.push(ch);
        }
    }

    num_string.parse::<usize>().unwrap()
}

fn process(filename: &str, new_style: bool, verbose: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}, {}", filename, if new_style { "new style" } else { "regular style" });

    let (column_starts, column_widths) = calc_columns(&lines);
    let (matrix, oper_mults) = create_matrix(&lines, &column_starts, &column_widths);

    let mut sum_of_results = 0usize;
    for column_index in 0..column_starts.len() {
        if oper_mults[column_index] {
            let mut result = 1usize;
            let row_index_max = if new_style { column_widths[column_index] } else { matrix.len() };
            for row_index in 0..row_index_max {
                let number = get_number_from_matrix(&matrix, column_index, row_index, new_style);
                result *= number;
            }

            if verbose {
                println!("Result of * in column {column_index} = {}", result);
            }

            sum_of_results += result;
        } else {
            let mut result = 0usize;
            let row_index_max = if new_style { column_widths[column_index] } else { matrix.len() };
            for row_index in 0..row_index_max {
                let number = get_number_from_matrix(&matrix, column_index, row_index, new_style);
                result += number;
            }

            if verbose {
                println!("Result of + in column {column_index} = {}", result);
            }

            sum_of_results += result;
        }
    }

    println!("  Sum of all results = {sum_of_results}");
}

fn main() {
    process("Example.txt", false, true);
    process("Input.txt", false, false);
    process("Example.txt", true, true);
    process("Input.txt", true, false);
}
