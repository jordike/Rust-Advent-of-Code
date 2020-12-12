use aoc::challenge;

mod day1;
mod day2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    challenge!(1, 1, day1::run()?);
    challenge!(2, 1, day2::part1()?);
    challenge!(2, 2, day2::part2()?);

    Ok(())
}