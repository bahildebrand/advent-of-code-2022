use std::{collections::HashSet, ops::Add};

use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

impl Cube {
    const DIRS: [(i64, i64, i64); 6] = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    fn adjacent(&self) -> impl Iterator<Item = Cube> + '_ {
        Self::DIRS.iter().map(|dir| *self + *dir)
    }
}

impl Add<(i64, i64, i64)> for Cube {
    type Output = Cube;

    fn add(self, rhs: (i64, i64, i64)) -> Self::Output {
        Cube {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
        }
    }
}

struct LavaFlow {
    cubes: HashSet<Cube>,
}

impl LavaFlow {
    fn calculate_surface_area(&self) -> usize {
        self.cubes
            .iter()
            .flat_map(|cube| cube.adjacent())
            .filter(|cube| !self.cubes.contains(cube))
            .count()
    }
}

fn parse(path: &str) -> Result<LavaFlow> {
    let input = std::fs::read_to_string(path)?;

    let mut cubes = HashSet::new();
    for line in input.lines() {
        let tokens = line.split(',').collect::<Vec<_>>();

        let x = tokens[0].parse::<i64>()?;
        let y = tokens[1].parse::<i64>()?;
        let z = tokens[2].parse::<i64>()?;

        cubes.insert(Cube { x, y, z });
    }

    Ok(LavaFlow { cubes })
}

pub fn day_18() -> Result<()> {
    let lava_flow = parse("input/day_18.txt")?;
    let surface_area = lava_flow.calculate_surface_area();

    println!("Day 18-1: {}", surface_area);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cube_surface() -> Result<()> {
        let lava_flow = parse("input/day_18_test.txt")?;

        let surface_area = lava_flow.calculate_surface_area();

        assert_eq!(surface_area, 64);

        Ok(())
    }
}
