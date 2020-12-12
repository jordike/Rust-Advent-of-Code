use aoc::challenge;

use year_2015::day1;
use year_2015::day2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    challenge!(1, 1, day1::run(None)?);
    challenge!(2, 1, day2::part1(None)?);
    challenge!(2, 2, day2::part2(None)?);

    Ok(())
}