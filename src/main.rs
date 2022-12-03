mod day_1;
mod day_2;

use anyhow::Result;

use day_1::day_1;
use day_2::day_2;

fn main() -> Result<()> {
    day_1()?;
    day_2()?;

    Ok(())
}
