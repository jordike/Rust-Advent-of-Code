use std::fs::read_to_string;
use aoc::{str_to_i32, read_input};

pub fn run(input: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input!(input, "input/2020/day1.txt");

    for line in input.lines() {
        let line_as_num = str_to_i32!(line);

        // Adding numbers above 2000 to another number
        // cannot sum up to 2000.
        if line_as_num >= 2000 {
            continue;
        }

        let number_needed = 2000 - line_as_num;

        let number_needed_string = number_needed.to_string();
        let number_needed_string = number_needed_string.as_str();

        // Test if the number is in the input file.
        if input.find(number_needed_string).is_some() {
            let added = line_as_num * str_to_i32!(number_needed);

            println!("Number 1: {}\tNumber 2: {}\tAdded: {}", line, number_needed, added);

            break;
        }
    }

    Ok(())
}