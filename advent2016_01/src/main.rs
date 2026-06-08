use adventlib::*;

fn process(filename: &str) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);


//    println!("  Total distance = {total_distance}, largest distance = {:?}, so distance of shortest route = {}", largest_distance.unwrap(), total_distance - largest_distance.unwrap());
}

fn main() {
    process("Example.txt");
    //process("Input.txt");
}
