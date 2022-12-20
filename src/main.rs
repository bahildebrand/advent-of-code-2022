mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_18;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use anyhow::Result;
use tracing::Level;

use day_1::day_1;
use day_10::day_10;
use day_11::day_11;
use day_12::day_12;
use day_13::day_13;
use day_14::day_14;
use day_15::day_15;
use day_16::day_16;
use day_18::day_18;
use day_2::day_2;
use day_3::day_3;
use day_4::day_4;
use day_5::day_5;
use day_6::day_6;
use day_7::day_7;
use day_8::day_8;
use day_9::day_9;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let root = tracing::span!(Level::INFO, "Root");
    let _enter = root.enter();

    day_1()?;
    day_2()?;
    day_3()?;
    day_4()?;
    day_5()?;
    day_6()?;
    day_7()?;
    day_8()?;
    day_9()?;
    day_10()?;
    day_11()?;
    day_12()?;
    day_13()?;
    day_14()?;
    day_15()?;
    day_16()?;
    day_18()?;

    Ok(())
}
