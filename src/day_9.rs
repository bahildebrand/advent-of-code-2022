use std::collections::{HashMap, HashSet};

use anyhow::{bail, Error, Result};

#[derive(Clone, Copy)]
enum Cmd {
    Up(i32),
    Right(i32),
    Left(i32),
    Down(i32),
}

impl TryFrom<&str> for Cmd {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = value.split(' ').collect::<Vec<_>>();

        if tokens.len() != 2 {
            bail!("Invalid command output");
        }

        let move_count = tokens[1].parse::<i32>()?;
        match tokens[0] {
            "U" => Ok(Cmd::Up(move_count)),
            "R" => Ok(Cmd::Right(move_count)),
            "L" => Ok(Cmd::Left(move_count)),
            "D" => Ok(Cmd::Down(move_count)),
            _ => bail!("Invalid move type"),
        }
    }
}

pub fn day_9() -> Result<()> {
    day_9_1()?;
    day_9_2()
}

fn day_9_1() -> Result<()> {
    let cmds = parse()?;

    let mut tail_map = vec![(0, 0)].into_iter().collect::<HashSet<_>>();
    let mut rope = [(0, 0); 2].to_vec();

    cmds.iter()
        .for_each(|cmd| iterate_moves(*cmd, &mut rope, &mut tail_map));

    println!("Day 9-1: {}", tail_map.len());

    Ok(())
}

fn day_9_2() -> Result<()> {
    let cmds = parse()?;

    let mut tail_map = vec![(0, 0)].into_iter().collect::<HashSet<_>>();
    let mut rope = [(0, 0); 10].to_vec();

    cmds.iter()
        .for_each(|cmd| iterate_moves(*cmd, &mut rope, &mut tail_map));

    println!("Day 9-2: {}", tail_map.len());

    Ok(())
}

fn cmd_to_move_step(cmd: Cmd) -> (i32, i32) {
    match cmd {
        Cmd::Up(_) => (0, 1),
        Cmd::Right(_) => (1, 0),
        Cmd::Left(_) => (-1, 0),
        Cmd::Down(_) => (0, -1),
    }
}

fn cmd_to_tail_pos(cmd: Cmd, head: (i32, i32)) -> (i32, i32) {
    match cmd {
        Cmd::Up(_) => (head.0, head.1 - 1),
        Cmd::Right(_) => (head.0 - 1, head.1),
        Cmd::Left(_) => (head.0 + 1, head.1),
        Cmd::Down(_) => (head.0, head.1 + 1),
    }
}

fn cmd_to_end_point(cmd: Cmd, start: (i32, i32)) -> (i32, i32) {
    match cmd {
        Cmd::Up(move_size) => (start.0, start.1 + move_size),
        Cmd::Right(move_size) => (start.0 + move_size, start.1),
        Cmd::Left(move_size) => (start.0 - move_size, start.1),
        Cmd::Down(move_size) => (start.0, start.1 - move_size),
    }
}

fn iterate_moves(cmd: Cmd, rope: &mut Vec<(i32, i32)>, tail_map: &mut HashSet<(i32, i32)>) {
    let head_end = cmd_to_end_point(cmd, rope[0]);

    while rope[0] != head_end {
        let move_step = cmd_to_move_step(cmd);
        for knot_idx in 0..(rope.len() - 1) {
            rope[knot_idx] = (
                rope[knot_idx].0 + move_step.0,
                rope[knot_idx].1 + move_step.1,
            );

            if !check_adjacent(rope[knot_idx], rope[knot_idx + 1]) {
                rope[knot_idx + 1] = cmd_to_tail_pos(cmd, rope[knot_idx]);
            }
        }

        tail_map.insert(*rope.last().unwrap());
    }
}

fn check_adjacent(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    let dirs = [
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (0, 0),
    ];

    dirs.iter()
        .any(|(x, y)| (head_pos.0 + x, head_pos.1 + y) == tail_pos)
}

fn parse() -> Result<Vec<Cmd>> {
    let input = std::fs::read_to_string("input/day_9.txt")?;

    input
        .lines()
        .map(|cmd_str| Cmd::try_from(cmd_str))
        .collect()
}
