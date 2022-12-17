use std::collections::{HashMap, HashSet};

use anyhow::Result;

pub fn day_16() -> Result<()> {
    Ok(())
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[derive(Debug)]
struct Volcano {
    valve_map: HashMap<String, Valve>,
}

impl Volcano {
    fn find_highest_pressure(&self, time: u32) -> u32 {
        let mut valve_stack = vec![];
        let cur_valve = self.valve_map.get(&"AA".to_string()).unwrap();

        self.find_highest_pressure_inner(0, &mut valve_stack, time, cur_valve, HashSet::new())
    }

    fn find_highest_pressure_inner(
        &self,
        mut pressure: u32,
        valve_stack: &mut Vec<String>,
        time: u32,
        cur_valve: &Valve,
        mut visited: HashSet<String>,
    ) -> u32 {
        println!("{} - {:?}", time, valve_stack);
        if time == 0 {
            return pressure;
        }

        if valve_stack.len() >= self.valve_map.len() - 1 {
            for _ in 0..time {
                pressure += self.tick_pressure(valve_stack);
            }

            return pressure;
        }

        visited.insert(cur_valve.name.clone());

        let iterate_pressure = |pressure: &u32,
                                valve_stack: &mut Vec<String>,
                                cur_valve: &Valve,
                                time: &u32,
                                visited: HashSet<String>| {
            let pressure = pressure + self.tick_pressure(valve_stack);
            cur_valve
                .tunnels
                .clone()
                .into_iter()
                .map(|valve_name| {
                    let cur_valve = self.valve_map.get(&valve_name).unwrap();

                    self.find_highest_pressure_inner(
                        pressure,
                        valve_stack,
                        time - 1,
                        cur_valve,
                        visited.clone(),
                    )
                })
                .max()
                .unwrap_or_default()
        };

        let mut high_pressure =
            pressure + iterate_pressure(&pressure, valve_stack, cur_valve, &time, visited.clone());

        if !valve_stack.contains(&cur_valve.name) {
            pressure += self.tick_pressure(valve_stack);
            valve_stack.push(cur_valve.name.clone());
            let time = time - 1;

            if time > 0 {
                high_pressure = std::cmp::max(
                    high_pressure,
                    iterate_pressure(&pressure, valve_stack, cur_valve, &time, visited.clone()),
                );
            }

            valve_stack.pop();
        } else {
            high_pressure = std::cmp::max(
                high_pressure,
                iterate_pressure(&pressure, valve_stack, cur_valve, &time, visited.clone()),
            );
        }

        visited.remove(&cur_valve.name);

        println!("Pressure: {}", high_pressure);

        high_pressure
    }

    fn tick_pressure(&self, valves: &[String]) -> u32 {
        valves
            .iter()
            .map(|valve_name| self.valve_map.get(valve_name).unwrap().flow_rate)
            .sum()
    }
}

fn parse(path: &str) -> Result<Volcano> {
    let input = std::fs::read_to_string(path)?;
    let mut valve_map = HashMap::new();

    for line in input.lines() {
        let tokens = line.split("; ").collect::<Vec<_>>();
        let rate_tokens = tokens[0].split(' ').collect::<Vec<_>>();
        let name = rate_tokens[1].to_string();
        let flow_rate = rate_tokens[4]
            .split('=')
            .nth(1)
            .expect("Rate value doesn't exist")
            .parse::<u32>()?;

        let mut tunnels = Vec::new();
        let tunnel_tokens = tokens[1].split(' ').collect::<Vec<_>>();
        for token in tunnel_tokens[4..].iter() {
            tunnels.push(token.replace(",", ""));
        }

        valve_map.insert(
            name.clone(),
            Valve {
                flow_rate,
                tunnels,
                name,
            },
        );
    }

    Ok(Volcano { valve_map })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valve_backtrack() -> Result<()> {
        let volcano = parse("input/day_16_test.txt")?;

        let pressure = volcano.find_highest_pressure(30);

        println!("{}", pressure);

        Ok(())
    }
}
