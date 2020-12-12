use aoc::str_to_i32;
use std::cmp::min;
use std::fs::read_to_string;

fn parse_line(line: String) -> (i32, i32, i32) {
    let dimensions: Vec<&str> = line.split("x").collect();
    let length = str_to_i32!(dimensions[0]);
    let width = str_to_i32!(dimensions[1]);
    let height = str_to_i32!(dimensions[2]);

    (length, width, height)
}

pub fn part1() -> Result<i32, Box<dyn std::error::Error>> {
    let file_content = read_to_string("input/2015/day2.txt").unwrap();

    let mut total_area = 0;

    for line in file_content.lines() {
        let dimension_string = String::from(line);
        let (length, width, height) = parse_line(dimension_string);

        let area_front_back = 2 * length * width;
        let area_right_left = 2 * width * height;
        let area_top_bottom = 2 * height * length;

        let smallest_side = min(length * width, min(width * height, height * length));

        let area = area_front_back + area_right_left + area_top_bottom + smallest_side;

        total_area += area;
    }

    println!("Total area: {}", total_area);

    Ok(total_area)
}

pub fn part2() -> Result<i32, Box<dyn std::error::Error>> {
    let file_content = read_to_string("input/2015/day2.txt").unwrap();

    let mut total_ribbon_length = 0;

    for line in file_content.lines() {
        let dimension_string = String::from(line);
        let (length, width, height) = parse_line(dimension_string);

        let ribbon_wrap = length * 2 + width * 2;
        let ribbon_bow = length * width * height;
        let ribbon_length = ribbon_wrap + ribbon_bow;

        total_ribbon_length += ribbon_length;

        println!("Parsed: {:?}\tFeet of ribbon: {}", [length, width, height], ribbon_length);
    }

    println!("Total feet of ribbon: {}", total_ribbon_length);

    Ok(total_ribbon_length)
}
