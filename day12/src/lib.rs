use log::debug;
use regex::Regex;
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
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,

    instructions: Vec<Instruction>,
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

    fn analyse_part1(&self) -> Option<i64> {
        use Instruction::*;

        let mut orientation = 90_i64;
        let mut x = 0_i64;
        let mut y = 0_i64;
        for instruction in &self.instructions {
            match instruction {
                North(d) => y += *d as i64,
                South(d) => y -= *d as i64,
                East(d) => x += *d as i64,
                West(d) => x -= *d as i64,
                Left(d) => orientation -= *d as i64,
                Right(d) => orientation += *d as i64,
                Forward(d) => match orientation {
                    0 => y += *d as i64,
                    90 => x += *d as i64,
                    180 => y -= *d as i64,
                    270 => x -= *d as i64,
                    _ => unreachable!(),
                },
            };
            while orientation < 0 {
                orientation += 360;
            }
            while orientation >= 360 {
                orientation -= 360;
            }
        }
        debug!("({}, {}) {}", x, y, orientation);
        Some(x.abs() + y.abs())
    }

    fn analyse_part2(&self) -> Option<i64> {
        use Instruction::*;

        let mut waypoint_x = 10_i64;
        let mut waypoint_y = 1_i64;
        let mut x = 0_i64;
        let mut y = 0_i64;

        for instruction in &self.instructions {
            match instruction {
                North(d) => waypoint_y += *d as i64,
                South(d) => waypoint_y -= *d as i64,
                East(d) => waypoint_x += *d as i64,
                West(d) => waypoint_x -= *d as i64,
                Left(d) => {
                    let mut d = *d;
                    while d > 0 {
                        let t = waypoint_x;
                        waypoint_x = -waypoint_y;
                        waypoint_y = t;
                        d -= 90;
                    }
                }
                Right(d) => {
                    let mut d = *d;
                    while d > 0 {
                        let t = waypoint_x;
                        waypoint_x = waypoint_y;
                        waypoint_y = -t;
                        d -= 90;
                    }
                }
                Forward(d) => {
                    x += (*d as i64) * waypoint_x;
                    y += (*d as i64) * waypoint_y;
                }
            }
        }
        debug!("({}, {}) ({}, {})", x, y, waypoint_x, waypoint_y);
        Some(x.abs() + y.abs())
    }
}

impl AddAssign<Instruction> for Solution {
    fn add_assign(&mut self, rhs: Instruction) {
        self.instructions.push(rhs);
    }
}

#[derive(Debug)]
enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?P<mode>.)(?P<amount>\d+)$").unwrap();

        let capt = re.captures(s).unwrap();
        let amount = capt.name("amount").unwrap().as_str().parse().unwrap();
        let instruction = match capt.name("mode").unwrap().as_str() {
            "N" => Self::North(amount),
            "S" => Self::South(amount),
            "E" => Self::East(amount),
            "W" => Self::West(amount),
            "L" => Self::Left(amount),
            "R" => Self::Right(amount),
            "F" => Self::Forward(amount),
            _ => unreachable!(),
        };

        Ok(instruction)
    }
}
