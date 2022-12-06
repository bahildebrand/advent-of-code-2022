use std::io::{BufRead, Lines};
use std::{fs::File, io::BufReader};

use anyhow::{Error, Result};
use itertools::Itertools;

#[derive(Debug, Default)]
struct CrateYard {
    piles: Vec<Vec<char>>,
}

impl CrateYard {
    fn push(&mut self, elf_crate: char, idx: usize) {
        while idx >= self.piles.len() {
            self.piles.push(Vec::new());
        }
        self.piles[idx].push(elf_crate);
    }

    fn move_crate(&mut self, count: usize, src: usize, dest: usize) -> Result<()> {
        let src = src - 1;
        let dest = dest - 1;

        for _ in 0..count {
            let move_char = self.piles[src]
                .pop()
                .ok_or_else(|| Error::msg(format!("More crates popped than in pile: {}", count)))?;
            self.piles[dest].push(move_char);
        }

        Ok(())
    }

    fn move_crate_n(&mut self, count: usize, src: usize, dest: usize) -> Result<()> {
        let src = src - 1;
        let dest = dest - 1;

        let split = self.piles[src].len() - count;

        let crates = self.piles[src].split_off(split);
        self.piles[dest].extend(crates);

        Ok(())
    }
}

fn build_crate_yard(lines: &mut Lines<BufReader<File>>) -> Result<CrateYard> {
    let mut crate_yard = CrateYard::default();

    let mut yard_lines = Vec::new();
    let mut next_line = lines.next().unwrap()?;
    while !next_line.chars().nth(1).unwrap().is_numeric() {
        yard_lines.push(next_line);
        next_line = lines.next().unwrap()?;
    }

    for line in yard_lines.iter_mut().rev() {
        let mut chunk_index = 0;
        for chars in &line.chars().chunks(4) {
            let char_string = chars.collect::<Vec<_>>();
            let crate_char = char_string
                .get(1)
                .ok_or_else(|| Error::msg("Invalid crate line"))?;

            if !crate_char.is_alphabetic() {
                chunk_index += 1;
                continue;
            } else {
                crate_yard.push(*crate_char, chunk_index);
                chunk_index += 1;
            }
        }
    }

    // Kind of silly, but I'm tired
    lines.next();

    Ok(crate_yard)
}

pub fn day_5() -> Result<()> {
    day_5_1()?;
    day_5_2()
}

fn day_5_1() -> Result<()> {
    let file = File::open("input/day_5.txt")?;
    let mut lines = std::io::BufReader::new(file).lines();

    let mut crate_yard = build_crate_yard(&mut lines)?;

    for line in lines {
        let line = line?;
        let tokens = line.split(' ').collect::<Vec<_>>();
        let count = tokens[1].parse::<usize>()?;
        let src = tokens[3].parse::<usize>()?;
        let dest = tokens[5].parse::<usize>()?;

        crate_yard.move_crate(count, src, dest)?;
    }

    let mut top_str = String::new();
    for pile in crate_yard.piles {
        if let Some(crate_char) = pile.last() {
            top_str.push(*crate_char);
        }
    }

    println!("Day 5-1: {}", top_str);

    Ok(())
}

fn day_5_2() -> Result<()> {
    let file = File::open("input/day_5.txt")?;
    let mut lines = std::io::BufReader::new(file).lines();

    let mut crate_yard = build_crate_yard(&mut lines)?;

    for line in lines {
        let line = line?;
        let tokens = line.split(' ').collect::<Vec<_>>();
        let count = tokens[1].parse::<usize>()?;
        let src = tokens[3].parse::<usize>()?;
        let dest = tokens[5].parse::<usize>()?;

        crate_yard.move_crate_n(count, src, dest)?;
    }

    let mut top_str = String::new();
    for pile in crate_yard.piles {
        if let Some(crate_char) = pile.last() {
            top_str.push(*crate_char);
        }
    }

    println!("Day 5-2: {}", top_str);

    Ok(())
}
