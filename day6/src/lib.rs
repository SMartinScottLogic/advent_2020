use anyhow::Context;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).context(format!("loading '{}'", filename))?;

    let mut solution = Solution::default();
    let reader = BufReader::new(file);
    let mut group = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            solution.add_group(group);
            group = Vec::new();
        } else {
            group.push(line.to_string());
        }
    }
    solution.add_group(group);
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    groups: Vec<Vec<String>>,
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
}

impl Solution {
    pub fn analyse(&mut self) {
        let part1: usize = self
            .groups
            .iter()
            .map(|v| Self::seen_answers(v))
            .map(|v| v.len())
            .sum();
        let part2: usize = self
            .groups
            .iter()
            .map(|v| Self::all_answers(v))
            .map(|v| v.len())
            .sum();
        self.answer_part1 = Some(part1 as i64);
        self.answer_part2 = Some(part2 as i64);
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn add_group(&mut self, group: Vec<String>) {
        self.groups.push(group);
    }

    fn all_answers(group: &[String]) -> String {
        group
            .iter()
            .flat_map(|v| v.chars())
            .fold(HashMap::new(), |mut acc, v| {
                *acc.entry(v).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .filter(|(_k, v)| *v == group.len())
            .map(|(k, _v)| k)
            .collect()
    }

    fn seen_answers(group: &[String]) -> String {
        group.iter().flat_map(|v| v.chars()).unique().collect()
    }
}

#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }
}
