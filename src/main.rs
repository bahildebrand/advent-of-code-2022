mod day_1;

use anyhow::Result;

use day_1::{day_1_1, day_1_2};

fn main() -> Result<()> {
    println!("Day 1-1: {}", day_1_1()?);
    println!("Day 1-2: {}", day_1_2()?);

    Ok(())
}
