use std::cmp::Ordering;

use anyhow::{Error, Result};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, line_ending, u32};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, pair, separated_pair};
use nom::Parser;

#[derive(Debug, Clone)]
enum Val {
    Arr(Vec<Val>),
    Int(u32),
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Arr(a), Self::Arr(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Arr(_), Self::Int(_)) | (Self::Int(_), Self::Arr(_)) => false,
        }
    }
}

impl Eq for Val {}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = match (self, other) {
            (Val::Int(a), Val::Int(b)) => a.cmp(b),
            (Val::Int(a), Val::Arr(b)) => Val::Arr(vec![Val::Int(*a)]).cmp(&Val::Arr(b.clone())),
            (Val::Arr(a), Val::Int(b)) => Val::Arr(a.clone()).cmp(&Val::Arr(vec![Val::Int(*b)])),
            (Val::Arr(a), Val::Arr(b)) => a.cmp(b),
        };

        Some(ordering)
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn day_13() -> Result<()> {
    day_13_1()?;
    day_13_2()
}

fn day_13_1() -> Result<()> {
    let input = std::fs::read_to_string("input/day_13.txt")?;
    let result = sum_of_ordered_pairs(parse(&input)?);

    println!("Day 13-1: {}", result);

    Ok(())
}

fn day_13_2() -> Result<()> {
    let input = std::fs::read_to_string("input/day_13.txt")?;
    let result = decoder_key(parse(&input)?);

    println!("Day 13-2: {}", result);

    Ok(())
}

fn sum_of_ordered_pairs(pairs: Vec<(Val, Val)>) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, (val1, val2))| if val1 < val2 { Some(idx + 1) } else { None })
        .sum::<usize>()
}

fn decoder_key(pairs: Vec<(Val, Val)>) -> usize {
    let packet_1 = Val::Arr(vec![Val::Arr(vec![Val::Int(2)])]);
    let packet_2 = Val::Arr(vec![Val::Arr(vec![Val::Int(6)])]);

    let packets = pairs
        .iter()
        .flat_map(|(a, b)| [a, b])
        .chain([&packet_1, &packet_2])
        .sorted()
        .collect::<Vec<_>>();

    packets
        .iter()
        .enumerate()
        .filter_map(|(idx, packet)| {
            if **packet == packet_1 || **packet == packet_2 {
                Some(idx + 1)
            } else {
                None
            }
        })
        .product::<usize>()
}

fn parse_val(input: &str) -> nom::IResult<&str, Val> {
    alt((
        delimited(char('['), separated_list0(char(','), parse_val), char(']')).map(Val::Arr),
        u32.map(Val::Int),
    ))(input)
}

fn parse(input: &str) -> Result<Vec<(Val, Val)>> {
    let result = separated_list1(
        pair(line_ending, line_ending),
        separated_pair(parse_val, line_ending, parse_val),
    )(input)
    .map_err(|_| Error::msg("Failed to parse"))?
    .1;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_ordered_pairs() -> Result<()> {
        let input = std::fs::read_to_string("input/day_13_test.txt")?;
        let result = sum_of_ordered_pairs(parse(&input)?);

        assert_eq!(result, 13);

        Ok(())
    }

    #[test]
    fn test_decoder_key() -> Result<()> {
        let input = std::fs::read_to_string("input/day_13_test.txt")?;
        let result = decoder_key(parse(&input)?);

        assert_eq!(result, 140);

        Ok(())
    }
}
