use anyhow::Context;
use log::debug;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).context(format!("loading '{}'", filename))?;

    let mut solution = Solution::default();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        solution.add_pass(line);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    seat_codes: Vec<String>,
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
}

impl Solution {
    pub fn analyse(&mut self) {
        let ids = self
            .seat_codes
            .iter()
            .map(|pass| Self::get_id(pass))
            .collect::<HashSet<_>>();
        self.answer_part1 = ids.iter().map(|v| v.to_owned()).max();
        self.answer_part2 = None;
        for id in &ids {
            if !ids.contains(&(id + 1)) && ids.contains(&(id + 2)) {
                self.answer_part2 = Some(id + 1);
            }
        }
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn add_pass(&mut self, pass: &str) {
        self.seat_codes.push(pass.to_string());
    }

    fn search(
        code: impl Iterator<Item = char>,
        min: i32,
        max: i32,
        take_lower: char,
        take_upper: char,
    ) -> i32 {
        let mut min = min;
        let mut max = max;
        for c in code {
            let mid = (max - min + 1) / 2;
            if c == take_lower {
                max -= mid;
            } else if c == take_upper {
                min += mid;
            } else {
                panic!();
            }
            debug!("{} {} {}; {}", c, min, max, min);
        }
        min
    }

    fn get_id(pass: &str) -> i64 {
        (Self::get_row(pass) * 8 + Self::get_column(pass)) as i64
    }

    fn get_row(pass: &str) -> i32 {
        Self::search(pass.chars().take(7), 0, 127, 'F', 'B')
    }

    fn get_column(pass: &str) -> i32 {
        Self::search(pass.chars().skip(7), 0, 7, 'L', 'R')
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
    fn pass1() {
        let pass = "FBFBBFFRLR";
        let result_row = Solution::get_row(pass);
        let result_column = Solution::get_column(pass);
        assert_eq!(result_row, 44);
        assert_eq!(result_column, 5);
    }

    #[test]
    fn pass2() {
        let pass = "BFFFBBFRRR";
        let result_row = Solution::get_row(pass);
        let result_column = Solution::get_column(pass);
        assert_eq!(result_row, 70);
        assert_eq!(result_column, 7);
    }

    #[test]
    fn pass3() {
        let pass = "FFFBBBFRRR";
        let result_row = Solution::get_row(pass);
        let result_column = Solution::get_column(pass);
        assert_eq!(result_row, 14);
        assert_eq!(result_column, 7);
    }

    #[test]
    fn pass4() {
        let pass = "BBFFBBFRLL";
        let result_row = Solution::get_row(pass);
        let result_column = Solution::get_column(pass);
        assert_eq!(result_row, 102);
        assert_eq!(result_column, 4);
    }
}
