use anyhow::Context;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).context(format!("loading '{}'", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::default();
    for line in reader.lines() {
        let line = line?;
        solution.add(Password::from_str(&line).unwrap());
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    data: Vec<Password>,
    answer1: Option<i64>,
    answer2: Option<i64>,
}

impl Solution {
    pub fn analyse(&mut self) {
        self.answer1 = None;
        let mut num_valid_part1 = 0;
        let mut num_valid_part2 = 0;

        for p in &self.data {
            let freq = p.password.chars().fold(HashMap::new(), |mut acc, v| {
                *acc.entry(v).or_insert(0) += 1;
                acc
            });
            let num_seen = freq.get(&p.req_char).unwrap_or(&0).to_owned();
            if num_seen >= p.req_min && num_seen <= p.req_max {
                num_valid_part1 += 1;
            }
            let c1 = p
                .password
                .chars()
                .nth(p.req_min - 1)
                .map(|c| c == p.req_char)
                .unwrap_or(false);
            let c2 = p
                .password
                .chars()
                .nth(p.req_max - 1)
                .map(|c| c == p.req_char)
                .unwrap_or(false);

            if c1 != c2 {
                num_valid_part2 += 1;
            }
        }
        self.answer1 = Some(num_valid_part1);
        self.answer2 = Some(num_valid_part2);
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer2
    }
}

impl Solution {
    fn add(&mut self, password: Password) {
        self.data.push(password);
    }
}

#[derive(Debug, Default)]
struct Password {
    req_min: usize,
    req_max: usize,
    req_char: char,
    password: String,
}

impl FromStr for Password {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"^(?P<min_req>\d+)-(?P<max_req>\d+) (?P<req_char>.): (?P<password>.*)$")
                .unwrap();
        let capt = re.captures(s).unwrap();
        let req_min = capt.name("min_req").unwrap().as_str().parse().unwrap();
        let req_max = capt.name("max_req").unwrap().as_str().parse().unwrap();
        let req_char = capt
            .name("req_char")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();
        let password = capt.name("password").unwrap().as_str().to_string();

        Ok(Self {
            req_min,
            req_max,
            req_char,
            password,
        })
    }
}
