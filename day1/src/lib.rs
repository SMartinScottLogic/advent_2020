use anyhow::Context;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).context(format!("loading '{}'", filename))?;

    let reader = BufReader::new(file);

    let mut solution = Solution::default();
    for line in reader.lines() {
        let line = line?;
        solution.add(line.parse()?);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    data: HashSet<i64>,
    answer1: Option<i64>,
    answer2: Option<i64>,
}

impl Solution {
    pub fn analyse(&mut self) {
        self.answer1 = None;
        for (idx1, v1) in self.data.iter().enumerate() {
            if self.data.contains(&(2020 - v1)) {
                self.answer1 = Some(v1 * (2020 - v1));
            }
            for (idx2, v2) in self.data.iter().enumerate() {
                if idx2 <= idx1 {
                    continue;
                }
                if self.data.contains(&(2020 - (v1 + v2))) {
                    self.answer2 = Some(v1 * v2 * (2020 - (v1 + v2)));
                }
            }
        }
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer2
    }
}

impl Solution {
    fn add(&mut self, value: i64) {
        self.data.insert(value);
    }
}
