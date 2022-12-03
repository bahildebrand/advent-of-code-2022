mod day_1;
mod day_2;
mod day_3;

use anyhow::Result;

use day_1::day_1;
use day_2::day_2;
use day_3::day_3;

fn main() -> Result<()> {
    day_1()?;
    day_2()?;
    day_3()?;

    Ok(())
}
