use anyhow::{bail, Result};
use itertools::Itertools;
use tracing::debug;

type MonkeFn = fn(u64, u64) -> u64;

#[derive(Debug)]
enum MonkeOp {
    MonkeSelf(MonkeFn),
    MonkeOther((MonkeFn, u64)),
}

#[derive(Debug, Default)]
struct Test {
    div: u64,
    t_case: usize,
    f_case: usize,
}

#[derive(Debug)]
struct Monke {
    items: Vec<u64>,
    monke_op: MonkeOp,
    test: Test,
    touches: u64,
}

impl Monke {
    fn run_round(&mut self, worry: bool, divider: u64) -> Result<Vec<(u64, usize)>> {
        let mut results = Vec::new();

        for mut item in self.items.drain(..) {
            match self.monke_op {
                MonkeOp::MonkeSelf(op) => item = op(item, item),
                MonkeOp::MonkeOther((op, other)) => item = op(item, other),
            }

            if worry {
                item /= 3;
            }
            if item % self.test.div == 0 {
                results.push((item % divider, self.test.t_case));
            } else {
                results.push((item % divider, self.test.f_case));
            }

            self.touches += 1;
        }

        Ok(results)
    }
}

fn run_rounds(monkes: &mut Vec<Monke>, rounds: u32, worry: bool, divider: u64) -> Result<()> {
    for _ in 0..rounds {
        for idx in 0..monkes.len() {
            let moves = monkes[idx].run_round(worry, divider)?;
            for (item, monke_idx) in moves {
                monkes[monke_idx].items.push(item);
            }
        }
    }

    debug!("{:#?}", monkes);

    Ok(())
}

fn top_touches(monkes: &[Monke]) -> u64 {
    let sorted_touches = monkes
        .iter()
        .map(|monke| monke.touches)
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    sorted_touches[..2].iter().product::<u64>()
}

impl Default for Monke {
    fn default() -> Self {
        Self {
            items: Default::default(),
            test: Default::default(),
            monke_op: MonkeOp::MonkeSelf(|var1, _var2| var1),
            touches: u64::default(),
        }
    }
}

pub fn day_11() -> Result<()> {
    day_11_1()?;
    day_11_2()
}

fn day_11_1() -> Result<()> {
    let mut monkes = parse("input/day_11.txt")?;

    run_rounds(&mut monkes, 20, true, u64::MAX)?;

    println!("Day 11-1: {}", top_touches(&monkes));

    Ok(())
}

fn day_11_2() -> Result<()> {
    let mut monkes = parse("input/day_11.txt")?;

    let divider = monkes.iter().map(|monke| monke.test.div).product();
    run_rounds(&mut monkes, 10000, false, divider)?;

    println!("Day 11-1: {}", top_touches(&monkes));

    Ok(())
}

fn parse(path: &str) -> Result<Vec<Monke>> {
    let input = std::fs::read_to_string(path)?;

    let mut monkes = Vec::new();

    let mut cur_monke = None;
    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let tokens = line.split(':').collect::<Vec<_>>();
        if tokens[0].contains("Monkey") {
            cur_monke = Some(Monke::default());
            continue;
        }

        if let Some(monke) = &mut cur_monke {
            match tokens[0] {
                "Starting items" => {
                    let items = tokens[1].split(',').collect::<Vec<_>>();

                    for item in items {
                        let item_int = item.trim().parse::<u64>()?;
                        monke.items.push(item_int);
                    }
                }
                "Operation" => {
                    let tokens = tokens[1].trim().split(' ').collect::<Vec<_>>();

                    let monke_fn = match tokens[3] {
                        "*" => |var1, var2| var1 * var2,
                        "+" => |var1, var2| var1 + var2,
                        _ => bail!("Invalid op: {}", tokens[2]),
                    };

                    if tokens[4] == "old" {
                        monke.monke_op = MonkeOp::MonkeSelf(monke_fn);
                    } else {
                        let monke_int = tokens[4].parse::<u64>()?;
                        monke.monke_op = MonkeOp::MonkeOther((monke_fn, monke_int));
                    }
                }
                "Test" => {
                    let tokens = tokens[1].trim().split(' ').collect::<Vec<_>>();
                    monke.test.div = tokens[2].parse()?;
                }
                "If true" => {
                    monke.test.t_case =
                        tokens[1].trim().split(' ').collect::<Vec<_>>()[3].parse()?;
                }
                "If false" => {
                    monke.test.f_case =
                        tokens[1].trim().split(' ').collect::<Vec<_>>()[3].parse()?;

                    if let Some(monke) = cur_monke.take() {
                        monkes.push(monke);
                    }
                }
                _ => bail!("Invalid line: {}", tokens[0]),
            }
        }
    }

    Ok(monkes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_monke_worry() -> Result<()> {
        let mut monkes = parse("input/day_11_test.txt")?;

        run_rounds(&mut monkes, 20, true, u64::MAX)?;
        let touches = top_touches(&monkes);

        assert_eq!(touches, 10605);

        Ok(())
    }

    #[test]
    fn test_monke_no_worry() -> Result<()> {
        let mut monkes = parse("input/day_11_test.txt")?;

        let divider = monkes.iter().map(|monke| monke.test.div).product();
        run_rounds(&mut monkes, 10000, false, divider)?;
        let touches = top_touches(&monkes);

        assert_eq!(touches, 2713310158);

        Ok(())
    }
}
