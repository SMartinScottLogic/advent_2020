use log::debug;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::AddAssign;
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::new();

    for line in reader.lines() {
        let line = line?;
        solution += Instruction::from_str(&line)?;
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    instructions: Vec<Instruction>,

    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
}

impl Solution {
    fn new() -> Self {
        Self::default()
    }

    pub fn analyse(&mut self) {
        self.answer_part1 = self.analyse_part1();
        self.answer_part2 = self.analyse_part2();
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn analyse_part1(&self) -> Option<i64> {
        let mut mask = None;
        let mut values = HashMap::new();
        for instruction in &self.instructions {
            match instruction {
                Instruction::Mask(m) => mask = Some(m),
                Instruction::Set(addr, value) => {
                    let value = Instruction::apply_mask_part1(mask.unwrap(), *value);
                    values.insert(addr, value);
                }
            };
        }
        let mut total = 0_i64;
        for (_addr, value) in values {
            total += value;
        }
        Some(total)
    }

    fn analyse_part2(&self) -> Option<i64> {
        let mut mask = None;
        let mut values = HashMap::new();
        for instruction in &self.instructions {
            match instruction {
                Instruction::Mask(m) => mask = Some(m),
                Instruction::Set(addr, value) => {
                    let addresses = Instruction::apply_mask_part2(mask.unwrap(), *addr as i64);
                    for address in addresses {
                        values.insert(address, value);
                    }
                }
            }
        }
        let mut total = 0_i64;
        for (_addr, value) in values {
            total += value;
        }
        Some(total)
    }
}

impl AddAssign<Instruction> for Solution {
    fn add_assign(&mut self, rhs: Instruction) {
        self.instructions.push(rhs);
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Set(u64, i64),
}

impl Instruction {
    fn apply_mask_part1(mask: &str, value: i64) -> i64 {
        let mut value = value;
        for (idx, c) in mask.chars().rev().enumerate() {
            match c {
                'X' => {}
                '0' => {
                    let v = 1_i64 << idx;
                    let v = !v;
                    value &= v;
                }
                '1' => {
                    let v = 1_i64 << idx;
                    value |= v;
                }
                _ => unreachable!(),
            };
            debug!("{} {} {}", idx, c, value);
        }
        value
    }

    fn apply_mask_part2(mask: &str, value: i64) -> Vec<i64> {
        debug!("{:b}", value);
        let mut values = vec![value];
        for (idx, c) in mask.chars().rev().enumerate() {
            match c {
                'X' => {
                    // all values
                    let v = 1_i64 << idx;
                    if values.is_empty() {
                        values.push(0);
                        values.push(v);
                    } else {
                        let mut next_values = vec![true, false]
                            .iter()
                            .flat_map(|b| {
                                values.iter().map(move |value: &i64| match b {
                                    true => value | v,
                                    false => value & (!v),
                                })
                            })
                            .collect::<Vec<_>>();
                        values.clear();
                        values.append(&mut next_values);
                    }
                }
                '0' => {
                    // no change
                }
                '1' => {
                    // set to 1
                    let v = 1_i64 << idx;
                    values.iter_mut().for_each(|value| *value |= v);
                }
                _ => unreachable!(),
            };
            debug!(
                "{} {} {:?}",
                idx,
                c,
                values
                    .iter()
                    .map(|v| format!("{:b}", v))
                    .collect::<Vec<_>>()
            );
        }
        values
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("mask = ") {
            Ok(Self::Mask(s.to_string()))
        } else {
            let re = Regex::new(r"^mem\[(?P<addr>\d+)\] = (?P<value>\d+)$").unwrap();
            let capt = re.captures(s).unwrap();
            let addr = capt.name("addr").unwrap().as_str().parse::<u64>().unwrap();
            let value = capt.name("value").unwrap().as_str().parse::<i64>().unwrap();
            Ok(Self::Set(addr, value))
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }

    #[test]
    fn apply_mask_1() {
        let result = Instruction::apply_mask_part1("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11);
        assert_eq!(result, 73);
    }

    #[test]
    fn apply_mask_2() {
        let result = Instruction::apply_mask_part2("000000000000000000000000000000X1001X", 42);
        assert_eq!(result, vec![59, 58, 27, 26]);
    }
}
