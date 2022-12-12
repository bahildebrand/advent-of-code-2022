use std::collections::{HashSet, VecDeque};

use anyhow::{bail, Result};

struct TopoMap {
    grid: Vec<Vec<char>>,
}

impl TopoMap {
    const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn find_shortest_path(&self, include_a: bool) -> Result<u32> {
        let starting_locs = self.find_start(include_a)?;

        let mut shortest_path = u32::MAX;
        for (row, col) in starting_locs {
            if let Some(new_shortest) = self.find_shortest_path_inner(row, col)? {
                shortest_path = std::cmp::min(shortest_path, new_shortest);
            }
        }

        Ok(shortest_path)
    }

    fn find_shortest_path_inner(&self, start_row: usize, start_col: usize) -> Result<Option<u32>> {
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();

        let row_len = self.grid.len() as isize;
        let col_len = self.grid[0].len() as isize;

        q.push_back((start_row, start_col, 0));
        visited.insert((start_row, start_col));

        // The typecasts in here are brain dead. Can't be bothered to fix
        while let Some((row, col, path_len)) = q.pop_front() {
            let cur_char = if self.grid[row][col] == 'S' {
                'a'
            } else {
                self.grid[row][col]
            };

            for (x, y) in Self::DIRECTIONS {
                let new_row = row as isize + x;
                let new_col = col as isize + y;

                if new_row >= 0 && new_row < row_len && new_col >= 0 && new_col < col_len {
                    if visited.contains(&(new_row as usize, new_col as usize)) {
                        continue;
                    }

                    let next_char = self.grid[new_row as usize][new_col as usize];
                    if next_char == 'E' {
                        if cur_char == 'z' {
                            return Ok(Some(path_len + 1));
                        } else {
                            continue;
                        }
                    }

                    let cur_char_int = cur_char as u32;
                    let next_char_int = next_char as u32;

                    if cur_char_int + 1 == next_char_int || next_char_int <= cur_char_int {
                        q.push_back((new_row as usize, new_col as usize, path_len + 1));
                        visited.insert((new_row as usize, new_col as usize));
                    }
                }
            }
        }

        Ok(None)
    }

    fn find_start(&self, include_a: bool) -> Result<Vec<(usize, usize)>> {
        let mut starting_locs = Vec::new();
        for (row, line) in self.grid.iter().enumerate() {
            for (col, loc) in line.iter().enumerate() {
                if *loc == 'S' || (include_a && *loc == 'a') {
                    starting_locs.push((row, col));
                }
            }
        }

        if starting_locs.is_empty() {
            bail!("Start not found");
        }

        Ok(starting_locs)
    }
}

impl From<&str> for TopoMap {
    fn from(input: &str) -> Self {
        let mut grid = Vec::new();

        for line in input.lines() {
            let grid_line = line.chars().collect();

            grid.push(grid_line);
        }

        Self { grid }
    }
}

pub fn day_12() -> Result<()> {
    day_12_1()?;
    day_12_2()
}

fn day_12_1() -> Result<()> {
    let topo_map = parse("input/day_12.txt")?;
    let shortest_path = topo_map.find_shortest_path(false)?;

    println!("Day 12-1: {}", shortest_path);

    Ok(())
}

fn day_12_2() -> Result<()> {
    let topo_map = parse("input/day_12.txt")?;
    let shortest_path = topo_map.find_shortest_path(true)?;

    println!("Day 12-1: {}", shortest_path);

    Ok(())
}

fn parse(path: &str) -> Result<TopoMap> {
    let input = std::fs::read_to_string(path)?;

    Ok(TopoMap::from(input.as_str()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_shortest_path() -> Result<()> {
        let topo_map = parse("input/day_12_test.txt")?;

        let shortest_path = topo_map.find_shortest_path(false)?;

        assert_eq!(shortest_path, 31);

        Ok(())
    }

    #[test]
    fn test_find_shortest_path_include_a() -> Result<()> {
        let topo_map = parse("input/day_12_test.txt")?;

        let shortest_path = topo_map.find_shortest_path(true)?;

        assert_eq!(shortest_path, 29);

        Ok(())
    }
}
