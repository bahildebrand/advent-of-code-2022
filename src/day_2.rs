use std::cmp::Ordering;
use std::fs::File;
use std::io::BufRead;

use anyhow::{bail, Error, Result};

#[derive(PartialEq, Eq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn value(&self) -> u64 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    fn losing_play(&self) -> Self {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }

    fn winning_play(&self) -> Self {
        match self {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
            Rps::Scissors => Rps::Rock,
        }
    }
}

impl TryFrom<&str> for Rps {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Rps::Rock),
            "B" | "Y" => Ok(Rps::Paper),
            "C" | "Z" => Ok(Rps::Scissors),
            _ => bail!("Invalid RPS Option: {}", value),
        }
    }
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Rps::Rock, Rps::Rock) => Some(Ordering::Equal),
            (Rps::Rock, Rps::Paper) => Some(Ordering::Less),
            (Rps::Rock, Rps::Scissors) => Some(Ordering::Greater),
            (Rps::Paper, Rps::Rock) => Some(Ordering::Greater),
            (Rps::Paper, Rps::Paper) => Some(Ordering::Equal),
            (Rps::Paper, Rps::Scissors) => Some(Ordering::Less),
            (Rps::Scissors, Rps::Rock) => Some(Ordering::Less),
            (Rps::Scissors, Rps::Paper) => Some(Ordering::Greater),
            (Rps::Scissors, Rps::Scissors) => Some(Ordering::Equal),
        }
    }
}

impl Ord for Rps {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("None never returned here")
    }
}

enum MatchOutcome {
    Win,
    Draw,
    Loss,
}

impl MatchOutcome {
    fn value(&self) -> u64 {
        match self {
            MatchOutcome::Win => 6,
            MatchOutcome::Draw => 3,
            MatchOutcome::Loss => 0,
        }
    }
}

impl TryFrom<&str> for MatchOutcome {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(MatchOutcome::Loss),
            "Y" => Ok(MatchOutcome::Draw),
            "Z" => Ok(MatchOutcome::Win),
            _ => bail!("Invalid match outcome: {}", value),
        }
    }
}

pub fn day_2() -> Result<()> {
    day_2_1()?;
    day_2_2()
}

fn day_2_1() -> Result<()> {
    let file = File::open("input/day_2.txt")?;
    let mut score = 0u64;

    let lines = std::io::BufReader::new(file).lines();
    for (line_num, line) in lines.enumerate() {
        let line = line?;
        let tokens = line.split(' ').collect::<Vec<_>>();
        if tokens.len() != 2 {
            bail!("Invalid number of choices on line {}", line_num + 1);
        }

        let opponent = Rps::try_from(tokens[0])?;
        let me = Rps::try_from(tokens[1])?;

        score += me.value();
        score += match me.cmp(&opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
    }

    println!("Day 2-1: {}", score);

    Ok(())
}

fn day_2_2() -> Result<()> {
    let file = File::open("input/day_2.txt")?;
    let mut score = 0u64;

    let lines = std::io::BufReader::new(file).lines();
    for (line_num, line) in lines.enumerate() {
        let line = line?;
        let tokens = line.split(' ').collect::<Vec<_>>();
        if tokens.len() != 2 {
            bail!("Invalid number of choices on line {}", line_num + 1);
        }

        let opponent = Rps::try_from(tokens[0])?;
        let outcome = MatchOutcome::try_from(tokens[1])?;

        score += outcome.value();
        match outcome {
            MatchOutcome::Win => score += opponent.winning_play().value(),
            MatchOutcome::Draw => score += opponent.value(),
            MatchOutcome::Loss => score += opponent.losing_play().value(),
        }
    }

    println!("Day 2-2: {}", score);

    Ok(())
}
