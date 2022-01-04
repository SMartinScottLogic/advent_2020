use log::{debug, trace};
use std::cmp::{max, min};
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::default();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim().parse().unwrap();
        solution.add_data(line);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    data: Vec<i64>,
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
    fn add_data(&mut self, data: i64) {
        self.data.push(data);
    }

    fn analyse_part1(&self) -> Option<i64> {
        let (_v, mut data) = self.data.iter().sorted().fold((0, HashMap::new()), |(old, mut acc), v| {
            let diff = v - old;
            *acc.entry(diff).or_insert(0) += 1;
            (*v, acc)
        });
        // Add final step of 3 from last adapter to device
        *data.entry(3).or_insert(0) += 1;
        debug!("data: {:?}", data);
        data.get(&1).and_then(|v1| data.get(&3).map(|v3| v1 * v3))
    }

    fn analyse_part2(&self) -> Option<i64> {
        let mut data = self.data.iter().sorted().map(|v| (v.to_owned(), true)).collect::<Vec<_>>();
        debug!("data: {:?}", data);
        self.analyse_part2_step(0, &mut data)
        /*
        for (idx, v) in data.iter().enumerate() {
            debug!("({} / {}, {:?})", idx, self.data.len(), v);
            if idx == self.data.len() - 1 {
                return Some(1);
            }
        }
        None
        */
    }

    fn analyse_part2_step(&self, idx: usize, data: &mut Vec<(i64, bool)>) -> Option<i64> {
        if idx == self.data.len() - 1 {
            trace!("data @ end {:?}", data);
            Some(1)
        } else {
            let active = self.analyse_part2_step(idx + 1, data).unwrap_or(0);
            let prev = match idx {
                0 => 0,
                _ => data.get(idx - 1).unwrap().0
            };
            let next = data.get(idx + 1).unwrap().0;
            let mut inactive = 0;
            if (next - prev) <= 3 {
            data.get_mut(idx).unwrap().1 = false;
            inactive = self.analyse_part2_step(idx+1, data).unwrap_or(0);
            data.get_mut(idx).unwrap().1 = true;
            }
            Some(active + inactive)
            
        }
    }
}
