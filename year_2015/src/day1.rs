use aoc::read_input;
use std::fs::read_to_string;

pub fn run(input: Option<String>) -> Result<i32, Box<dyn std::error::Error>> {
    let file_content = read_input!(input, "input/2015/day1.txt");

    let mut floor = 0;

    for character in file_content.chars() {
        if character == '(' {
            floor += 1;
        } else if character == ')' {
            floor -= 1;
        }
    }

    println!("Floor: {}", floor);

    Ok(floor)
}