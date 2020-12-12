use std::fs::read_to_string;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = read_to_string("input/2015/day1.txt")
        .expect("Could not open input file. Does it exist?");

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