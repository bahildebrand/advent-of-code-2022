mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use anyhow::Result;

use day_1::day_1;
use day_2::day_2;
use day_3::day_3;
use day_4::day_4;
use day_5::day_5;
use day_6::day_6;
use day_7::day_7;
use day_8::day_8;
use day_9::day_9;

fn main() -> Result<()> {
    day_1()?;
    day_2()?;
    day_3()?;
    day_4()?;
    day_5()?;
    day_6()?;
    day_7()?;
    day_8()?;
    day_9()?;

    Ok(())
}
