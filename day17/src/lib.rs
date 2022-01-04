use log::debug;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::default();

    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            solution.set(x as i64, y as i64, 0_i64, 0_i64, c);
        }
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    data: HashMap<(i64, i64, i64, i64), char>,
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
}

impl Solution {
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
    fn set(&mut self, x: i64, y: i64, z: i64, w: i64, c: char) {
        self.data.insert((x, y, z, w), c);
    }

    fn analyse_part1(&mut self) -> Option<i64> {
        let mut data = self.data.clone();
        for _pass in 1..=6 {
            data = Self::pass_part1(data);
        }
        Some(data.len() as i64)
    }

    fn analyse_part2(&self) -> Option<i64> {
        let mut data = self.data.clone();
        for _pass in 1..=6 {
            data = Self::pass_part2(data);
        }
        Some(data.len() as i64)
    }

    fn pass_part1(data: HashMap<(i64, i64, i64, i64), char>) -> HashMap<(i64, i64, i64, i64), char> {
        let mut count = HashMap::new();
        for ((x, y, z, w), c) in &data {
            if *c!='#' {
                continue;
            }
            for dz in -1..=1_i64 {
                for dy in -1..=1_i64 {
                    for dx in -1..=1_i64 {
                        if dx.abs() + dy.abs() + dz.abs() != 0 {
                            *count.entry((x+dx, y+dy, z+dz, w+0)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        let mut next_data = HashMap::new();
        for ((x, y, z, w), c) in count {
            let was_active = match data.get(&(x, y, z, w)) {
                Some('#') => true,
                _ => false,
            };
            if c == 3 || (was_active && c == 2) {
                next_data.insert((x, y, z, w), '#');
            }
        }
        next_data
    }

    fn pass_part2(data: HashMap<(i64, i64, i64, i64), char>) -> HashMap<(i64, i64, i64, i64), char> {
        let mut count = HashMap::new();
        for ((x, y, z, w), c) in &data {
            if *c!='#' {
                continue;
            }
            for dz in -1..=1_i64 {
                for dy in -1..=1_i64 {
                    for dx in -1..=1_i64 {
                        for dw in -1..=1_i64 {
                            if dx.abs() + dy.abs() + dz.abs() + dw.abs() != 0 {
                                *count.entry((x+dx, y+dy, z+dz, w+dw)).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
        let mut next_data = HashMap::new();
        for ((x, y, z, w), c) in count {
            let was_active = match data.get(&(x, y, z, w)) {
                Some('#') => true,
                _ => false,
            };
            if c == 3 || (was_active && c == 2) {
                next_data.insert((x, y, z, w), '#');
            }
        }
        next_data
    }
}
