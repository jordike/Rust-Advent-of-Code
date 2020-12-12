use aoc::challenge;

use year_2020::day1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    challenge!(1, 1, day1::run(None)?);

    Ok(())
}