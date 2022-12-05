use std::fs::File;
use std::io::BufRead;

use anyhow::{bail, Error, Result};

struct SectionRange {
    start: u32,
    end: u32,
}

impl SectionRange {
    fn contains(&self, other: &SectionRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl TryFrom<&str> for SectionRange {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = value.split('-').collect::<Vec<_>>();
        if tokens.len() != 2 {
            bail!("Invalid SectionRange string: {}", value);
        }

        let start = tokens[0].parse::<u32>()?;
        let end = tokens[1].parse::<u32>()?;

        Ok(Self { start, end })
    }
}

struct ElfPair {
    elf1: SectionRange,
    elf2: SectionRange,
}

impl TryFrom<String> for ElfPair {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens = value.split(',').collect::<Vec<_>>();
        if tokens.len() != 2 {
            bail!("Invalid SectionRange string: {}", value);
        }

        let elf1 = SectionRange::try_from(tokens[0])?;
        let elf2 = SectionRange::try_from(tokens[1])?;

        Ok(Self { elf1, elf2 })
    }
}

pub fn day_4() -> Result<()> {
    day_4_1()?;
    day_4_2()
}

fn day_4_1() -> Result<()> {
    fn full_overlap(pair: &ElfPair) -> bool {
        pair.elf1.contains(&pair.elf2) || pair.elf2.contains(&pair.elf1)
    }

    let sum = day_4_inner(full_overlap)?;

    println!("Day 4-1: {}", sum);

    Ok(())
}

fn day_4_2() -> Result<()> {
    fn partial_overlap(pair: &ElfPair) -> bool {
        pair.elf1.start <= pair.elf2.end && pair.elf2.start <= pair.elf1.end
    }

    let sum = day_4_inner(partial_overlap)?;

    println!("Day 4-2: {}", sum);

    Ok(())
}

type CompFunction = fn(&ElfPair) -> bool;

fn day_4_inner(comp_fn: CompFunction) -> Result<u32> {
    let file = File::open("input/day_4.txt")?;

    let mut sum = 0u32;
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        let line = line?;

        let elf_pair = ElfPair::try_from(line)?;
        if comp_fn(&elf_pair) {
            sum += 1;
        }
    }

    Ok(sum)
}
