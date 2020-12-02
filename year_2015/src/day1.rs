use std::vec::Vec;
use std::fs::read_to_string;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Please provide an input file!");
    }

    let input_path = &args[1]; // Ignore the first argument, because it is the executable.
    let file_content: String = read_to_string(input_path)?;

    let mut floor = 0;

    for character in file_content.chars() {
        if character == '(' {
            floor += 1;
        } else if character == ')' {
            floor -= 1;
        }
    }

    println!("Floor: {}", floor);

    Ok(())
}