use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Error, Result};
use tracing::debug;

#[derive(Debug, Clone, Copy)]
struct CycleCount(u32);

impl From<Op> for CycleCount {
    fn from(op: Op) -> Self {
        match op {
            Op::Noop => Self(1),
            Op::Addx(_) => Self(2),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Noop,
    Addx(i32),
}

impl TryFrom<&str> for Op {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = value.split(' ').collect::<Vec<_>>();

        let op = match tokens[0] {
            "noop" => Op::Noop,
            "addx" => Op::Addx(tokens[1].parse::<i32>()?),
            _ => bail!("Invalid op: {}", tokens[0]),
        };

        Ok(op)
    }
}

struct Sprite {
    pixels: Vec<i32>,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            pixels: vec![0, 1, 2],
        }
    }
}

impl Sprite {
    fn contains(&self, val: i32) -> bool {
        for pixel in &self.pixels {
            if *pixel == val {
                return true;
            }
        }

        return false;
    }

    fn set_pixel_location(&mut self, pixel: i32) {
        self.pixels = (pixel..(pixel + 3)).collect();
    }
}

#[derive(Debug)]
struct Cpu {
    ops: VecDeque<Op>,
    x: i32,
    display: Vec<Vec<char>>,
}

impl Cpu {
    fn parse_signal(&mut self) -> Result<i32> {
        let mut strength_map = (20u32..221u32)
            .step_by(40)
            .map(|cycle| (cycle, 0i32))
            .collect::<HashMap<_, _>>();

        let mut sprite = Sprite::default();

        let mut current_op = None;
        for cycle in 1u32..241u32 {
            if current_op.is_none() {
                if let Some(op) = self.ops.pop_front() {
                    current_op = Some((op, CycleCount::from(op)));
                }
            }

            let pixel = cycle as usize - 1;
            let col = pixel % 40;
            if sprite.contains(col as i32) {
                let row = pixel / 40;

                self.display[row][col] = '#';
            }

            if let Some(strength) = strength_map.get_mut(&cycle) {
                *strength = self.x * cycle as i32;

                debug!("X: {} - Strength: {} - Cycle: {}", self.x, *strength, cycle);
            }

            if let Some((op, cycle_count)) = &mut current_op {
                cycle_count.0 -= 1;
                match op {
                    Op::Noop => {}
                    Op::Addx(signal) => {
                        if cycle_count.0 == 0 {
                            self.x += *signal;
                            sprite.set_pixel_location(self.x - 1);
                        }
                    }
                }

                debug!("{:?} - {:?} - X:{}", op, cycle_count, self.x);

                if cycle_count.0 == 0 {
                    current_op = None;
                }
            }
        }

        Ok(strength_map.values().sum())
    }

    fn render_display(&self) -> String {
        let mut render = String::new();

        for line in &self.display {
            let mut line = String::from_iter(line);
            line.push('\n');
            render.push_str(&line);
        }

        render
    }
}

pub fn day_10() -> Result<()> {
    let mut cpu = parse()?;

    let signal_strength = cpu.parse_signal()?;
    println!("Day 10-1: {}", signal_strength);

    let display = cpu.render_display();
    print!("Day 10-2:\n{}", display);

    Ok(())
}

fn parse() -> Result<Cpu> {
    let input = std::fs::read_to_string("input/day_10.txt")?;

    parse_inner(input)
}

fn parse_inner(input: String) -> Result<Cpu> {
    let ops = input
        .lines()
        .map(|op_str| Op::try_from(op_str))
        .collect::<Result<VecDeque<_>>>()?;

    let mut display = Vec::with_capacity(6);
    let display_row = (0..40).map(|_| '.').collect::<Vec<_>>();
    for _ in 0..6 {
        display.push(display_row.clone());
    }

    Ok(Cpu { ops, x: 1, display })
}

#[cfg(test)]
mod test {
    use super::*;

    const DISPLAY_OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..\n\
                                  ###...###...###...###...###...###...###.\n\
                                  ####....####....####....####....####....\n\
                                  #####.....#####.....#####.....#####.....\n\
                                  ######......######......######......####\n\
                                  #######.......#######.......#######.....\n";

    #[test]
    fn test_signal() -> Result<()> {
        tracing_subscriber::fmt::init();

        let input = std::fs::read_to_string("input/day_10_test.txt")?;

        let mut cpu = parse_inner(input)?;

        let signal_strength = cpu.parse_signal()?;
        assert_eq!(signal_strength, 13140);

        let display = cpu.render_display();
        print!("{}", display);
        print!("{}", DISPLAY_OUTPUT);
        assert_eq!(display, DISPLAY_OUTPUT);

        Ok(())
    }
}
