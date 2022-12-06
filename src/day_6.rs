use std::{collections::HashMap, fs};

use anyhow::{bail, Result};

pub fn day_6() -> Result<()> {
    day_6_1()?;
    day_6_2()
}

// This code sucks and so do I
fn find_unique_seq(count: usize) -> Result<usize> {
    let code = fs::read_to_string("input/day_6.txt")?;

    let mut char_count = count;
    let mut char_map = HashMap::new();
    let mut window_start = 0;
    let mut window_end = count - 1;
    for ch in code[0..count].chars() {
        let entry = char_map.entry(ch).or_insert(0usize);
        *entry += 1;
    }

    if char_map.len() == count {
        return Ok(count);
    }

    for _ in count..code.len() {
        window_end += 1;
        {
            let new_char = code.chars().nth(window_end).unwrap();
            let entry = char_map.entry(new_char).or_insert(0usize);
            *entry += 1;
        }

        let old_char = code.chars().nth(window_start).unwrap();
        {
            let entry = char_map.entry(old_char).or_insert(0usize);
            *entry -= 1;
        }
        if *char_map.get(&old_char).unwrap() == 0 {
            char_map.remove(&old_char).unwrap();
        }
        window_start += 1;

        char_count += 1;

        if char_map.len() == count {
            break;
        }
    }

    if char_count == code.len() {
        bail!("Did not find start sequence")
    }

    Ok(char_count)
}

fn day_6_1() -> Result<()> {
    let char_count = find_unique_seq(4)?;

    println!("Day 6-1: {}", char_count);

    Ok(())
}

fn day_6_2() -> Result<()> {
    let char_count = find_unique_seq(14)?;

    println!("Day 6-2: {}", char_count);

    Ok(())
}
