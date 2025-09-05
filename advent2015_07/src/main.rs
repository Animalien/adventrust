use std::collections::HashMap;
use std::string::String;

use adventlib::*;

fn try_process_lines(lines: &Vec<String>, value_map: &mut HashMap<String, u16>) -> Vec<String> {
    let mut postponed_lines = Vec::<String>::new();

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
        let mut new_value;
        match first_arg.parse::<u16>() {
            Ok(value) => new_value = value,
            Err(_) => match value_map.get(first_arg) {
                Some(value) => new_value = *value,
                None => {
                    postponed_lines.push(line.to_string());
                    continue;
                },
            },
        };

        if notted {
            new_value = !new_value;
        }

        match token_iter.peek().unwrap() {
            &"->" => (),
            &"AND" => {
                token_iter.next();
                let second_arg = token_iter.next().unwrap();
                let second_arg_value;
                match second_arg.parse::<u16>() {
                    Ok(value) => second_arg_value = value,
                    Err(_) => match value_map.get(second_arg) {
                        Some(value) => second_arg_value = *value,
                        None => {
                            postponed_lines.push(line.to_string());
                            continue;
                        },
                    },
                };
                new_value = new_value & second_arg_value;
            },
            &"OR" => {
                token_iter.next();
                let second_arg = token_iter.next().unwrap();
                let second_arg_value;
                match second_arg.parse::<u16>() {
                    Ok(value) => second_arg_value = value,
                    Err(_) => match value_map.get(second_arg) {
                        Some(value) => second_arg_value = *value,
                        None => {
                            postponed_lines.push(line.to_string());
                            continue;
                        },
                    },
                };
                new_value = new_value | second_arg_value;
            },
            &"LSHIFT" => {
                token_iter.next();
                let second_arg = token_iter.next().unwrap();
                let second_arg_value;
                match second_arg.parse::<u16>() {
                    Ok(value) => second_arg_value = value,
                    Err(_) => match value_map.get(second_arg) {
                        Some(value) => second_arg_value = *value,
                        None => {
                            postponed_lines.push(line.to_string());
                            continue;
                        },
                    },
                };
                new_value = new_value << second_arg_value;
            },
            &"RSHIFT" => {
                token_iter.next();
                let second_arg = token_iter.next().unwrap();
                let second_arg_value;
                match second_arg.parse::<u16>() {
                    Ok(value) => second_arg_value = value,
                    Err(_) => match value_map.get(second_arg) {
                        Some(value) => second_arg_value = *value,
                        None => {
                            postponed_lines.push(line.to_string());
                            continue;
                        },
                    },
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

    postponed_lines
}

fn process(filename: &str, wires_to_print: &[&str], remap_source: Option<&str>, remap_dest: Option<&str>) {
    let mut lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}, with a final remap of {:?} to {:?}", filename, remap_source, remap_dest);

    let mut value_map = HashMap::<String, u16>::new();

    let mut postponed_lines = try_process_lines(&lines, &mut value_map);
    while !postponed_lines.is_empty() {
        postponed_lines = try_process_lines(&postponed_lines, &mut value_map);
    }

    if remap_source.is_some() && remap_dest.is_some() {
        let source_value = value_map.get(remap_source.unwrap()).unwrap();
        *lines.iter_mut().find(|s| { s.split(' ').rev().next().unwrap() == remap_dest.unwrap() }).unwrap() = format!("{source_value} -> {}", remap_dest.unwrap());

        value_map.clear();
        postponed_lines = try_process_lines(&lines, &mut value_map);
        while !postponed_lines.is_empty() {
            postponed_lines = try_process_lines(&postponed_lines, &mut value_map);
        }
    }
    
    for var_name in wires_to_print {
        println!("  {var_name} = {}", value_map.get(&var_name.to_string()).unwrap());
    }
}

fn main() {
    process("Examples.txt", &["d", "e", "f", "g", "h", "i", "x", "y"], None, None);
    process("Input.txt", &["a"], None, None);
    process("Input.txt", &["a"], Some("a"), Some("b"));
}
