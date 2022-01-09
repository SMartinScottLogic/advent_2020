use log::debug;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::Context;

pub fn load(payload: &str) -> anyhow::Result<Solution> {
    Solution::from_str(payload).context("Failed to parse")
}

#[derive(Debug, Default)]
pub struct Solution {
    input: Vec<i64>,
}

#[derive(Debug)]
struct Vec2 {
    last1: Option<usize>,
    last2: Option<usize>,
}

impl Vec2 {
    fn new() -> Self {
        Self {
            last1: None,
            last2: None,
        }
    }

    fn push(&mut self, v: usize) {
        self.last2 = self.last1;
        self.last1 = Some(v);
    }

    fn len(&self) -> usize {
        match (self.last1, self.last2) {
            (Some(_), Some(_)) => 2,
            (Some(_), None) => 1,
            (None, None) => 0,
            _ => unreachable!(),
        }
    }
}
impl Solution {
    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Option<i64> {
        let mut last = None;
        let mut seen = HashMap::new();
        for turn in 1..=2020 {
            if let Some(v) = self.input.get(turn - 1) {
                let v = *v;
                debug!("{}: {}", turn, v);
                last = Some(v);
                seen.entry(v).or_insert_with(Vec2::new).push(turn);
                continue;
            }
            if let Some(v) = last {
                let mut v = v;
                let last_seeings = seen.get(&v).unwrap();
                if last_seeings.len() < 2 {
                    v = 0;
                } else {
                    let last = last_seeings.last1.unwrap();
                    let prev = last_seeings.last2.unwrap();
                    v = (last - prev) as i64;
                }
                debug!("{}: {} {:?}", turn, v, last_seeings);
                last = Some(v);
                seen.entry(v).or_insert_with(Vec2::new).push(turn);
            }
        }
        last
    }

    pub fn answer_part2(&self) -> Option<i64> {
        let mut last = None;
        let mut seen = HashMap::new();
        for turn in 1..=30000000 {
            if let Some(v) = self.input.get(turn - 1) {
                let v = *v;
                debug!("{}: {}", turn, v);
                last = Some(v);
                seen.entry(v).or_insert_with(Vec2::new).push(turn);
                continue;
            }
            if let Some(v) = last {
                let mut v = v;
                let last_seeings = seen.get(&v).unwrap();
                if last_seeings.len() < 2 {
                    v = 0;
                } else {
                    let last = last_seeings.last1.unwrap();
                    let prev = last_seeings.last2.unwrap();
                    v = (last - prev) as i64;
                }
                debug!("{}: {} {:?}", turn, v, last_seeings);
                last = Some(v);
                seen.entry(v).or_insert_with(Vec2::new).push(turn);
            }
        }
        last
    }
}

impl FromStr for Solution {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = Vec::new();
        for part in s.trim().split(',') {
            let part = part.parse().unwrap();
            input.push(part);
        }
        Ok(Self { input })
    }
}
