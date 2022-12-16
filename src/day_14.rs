use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, u64};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::separated_pair;
use tracing::debug;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CavePoint {
    Rock,
    Sand,
    Air,
}

impl From<CavePoint> for char {
    fn from(point: CavePoint) -> Self {
        match point {
            CavePoint::Rock => '#',
            CavePoint::Sand => 'O',
            CavePoint::Air => '.',
        }
    }
}

struct Cave {
    grid: Vec<Vec<CavePoint>>,
    floor: usize,
}

impl Cave {
    const SAND_START: usize = 500;
    const SAND_DIRS: [(isize, isize); 2] = [(-1, 1), (1, 1)];
    const CAVE_WIDTH: usize = 800;

    fn check_free_space(&self, x: isize, y: isize) -> bool {
        if x >= 0 && x < self.grid.len() as isize && y >= 0 && y <= self.grid[0].len() as isize {
            self.grid[x as usize][y as usize] == CavePoint::Air
        } else {
            false
        }
    }

    fn drop_sand(&mut self, floor: bool) -> u64 {
        let mut counter = 0;

        if floor {
            for x in 0..self.grid.len() {
                self.grid[x][self.floor] = CavePoint::Rock;
            }
        }

        while self.drop_sand_inner() {
            counter += 1;
        }

        counter
    }

    fn drop_sand_inner(&mut self) -> bool {
        let mut cur_pos = Point {
            x: Self::SAND_START,
            y: 0,
        };

        if self.grid[cur_pos.x][cur_pos.y] == CavePoint::Sand {
            return false;
        }

        let cave_depth = self.grid[0].len();
        let mut resting = false;
        while cur_pos.y < cave_depth {
            let mut next_pos = Point {
                x: cur_pos.x,
                y: cur_pos.y + 1,
            };

            if next_pos.y >= cave_depth {
                break;
            }
            match self.grid[next_pos.x][next_pos.y] {
                CavePoint::Rock | CavePoint::Sand => {
                    let x_left = next_pos.x as isize + Self::SAND_DIRS[0].0;
                    let y_left = next_pos.y as isize; // + Self::SAND_DIRS[0].1;
                    let left = self.check_free_space(x_left, y_left);
                    let x_right = next_pos.x as isize + Self::SAND_DIRS[1].0;
                    let y_right = next_pos.y as isize; // + Self::SAND_DIRS[1].1;
                    let right = self.check_free_space(x_right, y_right);

                    if left {
                        next_pos.x = x_left as usize;
                        next_pos.y = y_left as usize;
                    } else if right {
                        next_pos.x = x_right as usize;
                        next_pos.y = y_right as usize;
                    } else {
                        self.grid[cur_pos.x][cur_pos.y] = CavePoint::Sand;
                        resting = true;
                        break;
                    }
                }
                CavePoint::Air => {}
            }

            cur_pos = next_pos;
        }

        resting
    }
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                write!(f, "{}", char::from(self.grid[x][y]))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl From<Vec<Vec<Point>>> for Cave {
    fn from(lines: Vec<Vec<Point>>) -> Self {
        let mut grid = vec![vec![CavePoint::Air; Self::CAVE_WIDTH]; Self::CAVE_WIDTH];
        let mut floor = 0;
        for line in lines {
            for window in line.windows(2) {
                if window[0].x != window[1].x {
                    let x_start = std::cmp::min(window[0].x, window[1].x);
                    let x_end = std::cmp::max(window[0].x, window[1].x);
                    let y = window[0].y;
                    floor = std::cmp::max(y, floor);
                    for x in x_start..(x_end + 1) {
                        grid[x][y] = CavePoint::Rock;
                    }
                }

                if window[0].y != window[1].y {
                    let y_start = std::cmp::min(window[0].y, window[1].y);
                    let y_end = std::cmp::max(window[0].y, window[1].y);
                    let x = window[0].x;
                    for y in y_start..(y_end + 1) {
                        floor = std::cmp::max(y, floor);
                        grid[x][y] = CavePoint::Rock;
                    }
                }
            }
        }

        floor += 2;
        Self { grid, floor }
    }
}

fn parse_val(input: &str) -> nom::IResult<&str, Point> {
    let (i, (x, y)) = separated_pair(u64, char(','), u64)(input)?;

    Ok((
        i,
        Point {
            x: x as usize,
            y: y as usize,
        },
    ))
}

fn parse_line(input: &str) -> nom::IResult<&str, Vec<Point>> {
    separated_list0(tag(" -> "), parse_val)(input)
}

fn parse(path: &str) -> Result<Cave> {
    let input = std::fs::read_to_string(path)?;
    let result = separated_list1(line_ending, parse_line)(&input).unwrap();
    let cave = Cave::from(result.1);

    Ok(cave)
}

pub fn day_14() -> Result<()> {
    day_14_1()?;
    day_14_2()
}

fn day_14_1() -> Result<()> {
    let mut cave = parse("input/day_14.txt")?;
    let sand = cave.drop_sand(false);

    println!("Day 14-1: {}", sand);
    debug!("{:?}", cave);

    Ok(())
}

fn day_14_2() -> Result<()> {
    let mut cave = parse("input/day_14.txt")?;
    let sand = cave.drop_sand(true);

    println!("Day 14-2: {}", sand);
    debug!("{:?}", cave);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let mut cave = parse("input/day_14_test.txt")?;

        let sand = cave.drop_sand(false);

        assert_eq!(sand, 24);

        Ok(())
    }
}
