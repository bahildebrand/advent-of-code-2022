use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;

use anyhow::{bail, Result};

struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
}

impl Rucksack {
    fn build_compartment_set(compartment: &str) -> HashSet<char> {
        compartment.chars().collect()
    }

    fn item_value(item: char) -> Result<u32> {
        if item.is_ascii_uppercase() {
            Ok(item as u32 - 65 + 27)
        } else if item.is_ascii_lowercase() {
            Ok(item as u32 - 96)
        } else {
            bail!("Invalid item {}", item);
        }
    }

    fn common_items(&self) -> Vec<char> {
        self.compartment1
            .intersection(&self.compartment2)
            .cloned()
            .collect()
    }

    fn whole_bag(&self) -> HashSet<char> {
        self.compartment1
            .union(&self.compartment2)
            .cloned()
            .collect()
    }
}

impl From<String> for Rucksack {
    fn from(contents: String) -> Self {
        let len = contents.len();

        let comp1 = &contents[0..(len / 2)];
        let comp2 = &contents[(len / 2)..len];

        Self {
            compartment1: Rucksack::build_compartment_set(comp1),
            compartment2: Rucksack::build_compartment_set(comp2),
        }
    }
}

pub fn day_3() -> Result<()> {
    day_3_1()?;
    day_3_2()
}

fn day_3_1() -> Result<()> {
    let file = File::open("input/day_3.txt")?;

    let mut sum = 0u32;
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        let line = line?;
        let rucksack = Rucksack::from(line);

        let common_items = rucksack.common_items();
        sum += common_items
            .iter()
            .map(|item| Rucksack::item_value(*item).unwrap())
            .sum::<u32>();
    }

    println!("Day 3-1: {}", sum);

    Ok(())
}

fn day_3_2() -> Result<()> {
    let file = File::open("input/day_3.txt")?;

    let mut sum = 0u32;
    let lines = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    for chunk in lines.chunks(3) {
        if chunk.len() != 3 {
            bail!("Input file lines not divisble by 3");
        }

        let rucksacks = chunk
            .iter()
            .map(|items| Rucksack::from(items.clone()))
            .collect::<Vec<_>>();

        let mut intersect = rucksacks[0].whole_bag();
        for rucksack in rucksacks[1..3].iter() {
            intersect = intersect
                .intersection(&rucksack.whole_bag())
                .cloned()
                .collect();
        }

        let final_val = intersect.into_iter().collect::<Vec<_>>();
        if final_val.len() != 1 {
            bail!(
                "Found more than 1 common item amongst three elves: {:?}",
                final_val
            );
        }

        sum += Rucksack::item_value(final_val[0])?;
    }

    println!("Day 3-2: {}", sum);

    Ok(())
}
