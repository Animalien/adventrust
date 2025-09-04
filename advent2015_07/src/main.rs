use std::collections::HashMap;
use std::string::String;

use adventlib::*;

fn process(filename: &str, wires_to_print: &[&str]) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    let mut value_map = HashMap::<String, u16>::new();
    for line in lines {
        let mut token_iter = line.split(' ').peekable();

        let mut notted = false;
        match token_iter.peek() {
            Some(&"NOT") => {
                notted = true;
                token_iter.next();
            },
            _ => ()
        }

        let first_arg = token_iter.next().unwrap();
        let mut new_value = match first_arg.parse::<u16>() {
            Ok(value) => value,
            Err(_) => *value_map.get(first_arg).unwrap(),
        };

        if notted {
            new_value = !new_value;
        }

        match token_iter.peek().unwrap() {
            &"->" => (),
            &"AND" => {
                token_iter.next();
                let second_arg = token_iter.next().unwrap();
                let second_arg_value = match second_arg.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => *value_map.get(second_arg).unwrap(),
                };
                new_value = new_value & second_arg_value;
            },
            &"OR" => {
                token_iter.next();
                let second_arg = token_iter.next().unwrap();
                let second_arg_value = match second_arg.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => *value_map.get(second_arg).unwrap(),
                };
                new_value = new_value | second_arg_value;
            },
            &"LSHIFT" => {
                token_iter.next();
                let second_arg = token_iter.next().unwrap();
                let second_arg_value = match second_arg.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => *value_map.get(second_arg).unwrap(),
                };
                new_value = new_value << second_arg_value;
            },
            &"RSHIFT" => {
                token_iter.next();
                let second_arg = token_iter.next().unwrap();
                let second_arg_value = match second_arg.parse::<u16>() {
                    Ok(value) => value,
                    Err(_) => *value_map.get(second_arg).unwrap(),
                };
                new_value = new_value >> second_arg_value;
            },
            _ => panic!("invalid argument!"),
        }

        let next_token = token_iter.next().unwrap();
        assert_eq!(next_token, "->");

        let destination = token_iter.next().unwrap();

        value_map.insert(destination.to_string(), new_value);
    }

    for var_name in wires_to_print {
        println!("  {var_name} = {}", value_map.get(&var_name.to_string()).unwrap());
    }
}

fn main() {
    process("Examples.txt", &["d", "e", "f", "g", "h", "i", "x", "y"]);
    process("Input.txt", &["a"]);
}
