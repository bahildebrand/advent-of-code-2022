use std::collections::HashSet;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct SensorPair {
    sensor: Point,
    beacon: Point,
    distance: i64,
}

impl SensorPair {
    fn check_point_in_range(&self, point: Point) -> bool {
        taxi_cab_distance(self.sensor, point) <= self.distance
    }
}

fn max_x(sensors: &[SensorPair]) -> i64 {
    sensors.iter().map(|pair| pair.sensor.x).max().unwrap()
}

fn check_at_y(pairs: &Vec<SensorPair>, y: i64) -> u64 {
    let max_x = max_x(pairs) * 2;
    let min_x = -max_x;
    let mut count = 0;
    let beacon_set: HashSet<Point> = pairs.iter().map(|pair| pair.beacon).collect();

    for x in min_x..max_x {
        let point = Point { x, y };
        if beacon_set.contains(&point) {
            continue;
        }

        for pair in pairs {
            if pair.check_point_in_range(point) {
                count += 1;
                break;
            }
        }
    }

    count
}

fn parse_point(token: &str) -> Result<Point> {
    let tokens = token.split('=').collect::<Vec<_>>();
    let x = tokens[1].split(", ").collect::<Vec<_>>()[0].parse::<i64>()?;
    let y = tokens[2].parse::<i64>()?;

    Ok(Point { x, y })
}

fn parse(path: &str) -> Result<Vec<SensorPair>> {
    let input = std::fs::read_to_string(path)?;
    let mut sensors = Vec::new();
    for line in input.lines() {
        let tokens = line.split(':').collect::<Vec<_>>();
        let sensor = parse_point(tokens[0])?;
        let beacon = parse_point(tokens[1])?;

        let distance = taxi_cab_distance(sensor, beacon);

        sensors.push(SensorPair {
            sensor,
            beacon,
            distance,
        });
    }

    Ok(sensors)
}

fn taxi_cab_distance(point1: Point, point2: Point) -> i64 {
    (point1.x - point2.x).abs() + (point1.y - point2.y).abs()
}

fn build_zone_set(pairs: &Vec<SensorPair>) -> HashSet<Point> {
    let mut set = HashSet::new();
    let dirs = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
    for pair in pairs {
        let possible_dist = pair.distance + 1;
        for i in 0..(possible_dist + 1) {
            for (x_sign, y_sign) in dirs {
                let x = pair.sensor.x + i * x_sign;
                let y = pair.sensor.y + possible_dist - i * y_sign;
                set.insert(Point { x, y });
            }
        }
    }

    set
}

fn tuning_frequency(zone_set: HashSet<Point>, pairs: &[SensorPair]) -> i64 {
    let ret_point = zone_set
        .iter()
        .filter(|point| !pairs.iter().all(|pair| pair.check_point_in_range(**point)))
        .collect::<Vec<_>>();

    // assert_eq!(ret_point.len(), 1);

    ret_point[0].x * 4000000 + ret_point[0].y
}

pub fn day_15() -> Result<()> {
    day_15_1()?;
    day_15_2()
}

fn day_15_1() -> Result<()> {
    let pairs = parse("input/day_15_test.txt")?;

    let count = check_at_y(&pairs, 10);
    println!("Day 15-1: {}", count);

    Ok(())
}

fn day_15_2() -> Result<()> {
    let pairs = parse("input/day_15_test.txt")?;

    let zone_set = build_zone_set(&pairs);
    let freq = tuning_frequency(zone_set, &pairs);

    println!("Day 15-2: {}", freq);

    Ok(())
}
