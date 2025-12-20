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

    let mut matrix = Vec::<Vec::<usize>>::new();
    let mut column_oper_is_mult = Vec::<bool>::new();
    for line in &lines {
        let mut new_matrix_line = Vec::<usize>::new();
        let mut line_iter = line.split_whitespace().peekable();

        if line_iter.peek().unwrap().parse::<usize>().is_ok() {
            while let Some(item) = line_iter.next() {
                new_matrix_line.push(item.parse::<usize>().unwrap());
            }
            matrix.push(new_matrix_line);
        } else {
            while let Some(item) = line_iter.next() {
                column_oper_is_mult.push(item == "*");
            }
            break;
        }
    }

    let mut sum_of_results = 0usize;
    for column_index in 0..column_oper_is_mult.len() {
        if column_oper_is_mult[column_index] {
            let mut result = 1usize;
            for row_index in 0..matrix.len() {
                result *= matrix[row_index][column_index];
            }

            if verbose {
                println!("Result of * in column {column_index} = {}", result);
            }

            sum_of_results += result;
        } else {
            let mut result = 0usize;
            for row_index in 0..matrix.len() {
                result += matrix[row_index][column_index];
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
    process("Example.txt", true);
    process("Input.txt", false);
}
