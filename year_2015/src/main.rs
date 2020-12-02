use aoc::challenge;

mod day1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    challenge!(1, day1::run()?);

    Ok(())
}