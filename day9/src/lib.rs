use log::debug;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        let v = self
            .data
            .iter()
            .scan(Vec::new(), |state: &mut Vec<i64>, v| {
                let last_state = state.iter().map(|v| v.to_owned()).collect::<HashSet<_>>();
                state.push(*v);
                if state.len() > 25 {
                    state.drain(0..=0);
                    Some((*v, last_state))
                } else {
                    Some((0, HashSet::new()))
                }
            })
            .filter(|(_v, state)| !state.is_empty())
            .collect::<Vec<_>>();

        v.iter()
            .find(|(value, state)| !state.iter().any(|v| state.contains(&(value - v))))
            .map(|(v, _state)| v.to_owned())
    }

    fn analyse_part2(&self) -> Option<i64> {
        let v = self
            .data
            .iter()
            .scan(Vec::new(), |state: &mut Vec<i64>, v| {
                let last_state = state.iter().map(|v| v.to_owned()).collect::<HashSet<_>>();
                state.push(*v);
                if state.len() > 25 {
                    state.drain(0..=0);
                    Some((*v, last_state))
                } else {
                    Some((0, HashSet::new()))
                }
            })
            .filter(|(_v, state)| !state.is_empty())
            .collect::<Vec<_>>();

        let probe = v
            .iter()
            .find(|(value, state)| !state.iter().any(|v| state.contains(&(value - v))))
            .map(|(v, _state)| v.to_owned())
            .unwrap();

        for (idx, _v) in self.data.iter().enumerate() {
            debug!("start idx: {}", idx);
            let mut total = 0;
            let mut min_value = None;
            let mut max_value = None;
            for i in idx..self.data.len() {
                total += self.data[i];
                min_value = match min_value {
                    None => Some(self.data[i]),
                    Some(v) => Some(min(v, self.data[i])),
                };
                max_value = match max_value {
                    None => Some(self.data[i]),
                    Some(v) => Some(max(v, self.data[i])),
                };
                if total == probe {
                    debug!("matched {} to {} {:?}-{:?}", idx, i, min_value, max_value);
                    return min_value.and_then(|v| max_value.map(|m| m + v));
                }
                if total > probe {
                    debug!("break {}", idx);
                    break;
                }
            }
        }
        None
    }
}
