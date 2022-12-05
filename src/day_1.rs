use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufRead;

use anyhow::Result;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    calories: u64,
}

impl Elf {
    fn push_food(&mut self, food: u64) {
        self.calories += food;
    }
}

fn build_elf_set() -> Result<BTreeSet<Elf>> {
    let file = File::open("input/day_1.txt")?;

    let mut elves = BTreeSet::new();
    let mut elf = Elf::default();
    let lines = std::io::BufReader::new(file).lines();
    for line in lines.flatten() {
        if line.is_empty() {
            let old_elf = std::mem::take(&mut elf);
            elves.insert(old_elf);
        } else {
            let calories = line.parse::<u64>()?;
            elf.push_food(calories);
        }
    }

    Ok(elves)
}

pub fn day_1() -> Result<()> {
    day_1_1()?;
    day_1_2()
}

pub fn day_1_1() -> Result<()> {
    let elves = build_elf_set()?;

    let calories = elves.into_iter().last().unwrap().calories;
    println!("Day 1-1: {}", calories);

    Ok(())
}

pub fn day_1_2() -> Result<()> {
    let elves = build_elf_set()?;

    let mut elf_iter = elves.into_iter().rev();
    let mut top_3_cals = 0;
    for _ in 0..3 {
        let elf = elf_iter.next().unwrap();
        top_3_cals += elf.calories;
    }

    println!("Day 1-2: {}", top_3_cals);

    Ok(())
}
