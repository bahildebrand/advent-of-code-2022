mod day_1;
mod day_2;

use anyhow::Result;

use day_1::{day_1_1, day_1_2};
use day_2::day_2;

fn main() -> Result<()> {
    println!("Day 1-1: {}", day_1_1()?);
    println!("Day 1-2: {}", day_1_2()?);

    day_2()?;

    Ok(())
}
