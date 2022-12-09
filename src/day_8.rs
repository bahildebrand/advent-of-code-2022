use itertools::Itertools;

use anyhow::Result;

pub fn day_8() -> Result<()> {
    day_8_1();
    day_8_2();

    Ok(())
}

fn parse() -> Vec<Vec<u32>> {
    let input = std::fs::read_to_string("input/day_8.txt").unwrap();

    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();

    let (up, down) = column.split_at(y);
    let (left, right) = row.split_at(x);

    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    [up, down, left, right]
}

pub fn day_8_1() {
    let trees = parse();
    let len = trees.len();

    let visible_trees = (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = trees[y][x];
            directions(&trees, x, y)
                .iter()
                .map(|direction| direction.iter().all(|h| *h < height))
                .any(|visible| visible)
        })
        .filter(|visible| *visible)
        .count()
        + (len - 1) * 4;

    println!("Day 8-1: {}", visible_trees);
}

pub fn day_8_2() {
    let trees = parse();
    let len = trees.len();

    let scenic_score = (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = trees[y][x];
            directions(&trees, x, y)
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .position(|h| *h >= height)
                        .map(|p| p + 1)
                        .unwrap_or_else(|| direction.len())
                })
                .product::<usize>()
        })
        .max()
        .unwrap();

    println!("Day 8-2: {}", scenic_score);
}
