use anyhow::Context;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).context(format!("loading '{}'", filename))?;

    let mut solution = Solution::default();
    let reader = BufReader::new(file);
    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            solution.set(x, y, c);
        }
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    map: HashMap<(usize, usize), char>,
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,

    max_x: usize,
    max_y: usize,
}

impl Solution {
    pub fn analyse(&mut self) {
        self.answer_part1 = Some(self.hit_trees(3, 1));

        let mut hit_trees = 1;
        for (dx, dy) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            hit_trees *= self.hit_trees(dx, dy);
        }
        self.answer_part2 = Some(hit_trees);
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn set(&mut self, x: usize, y: usize, c: char) {
        *self.map.entry((x, y)).or_default() = c;
        self.max_x = max(self.max_x, x);
        self.max_y = max(self.max_y, y);
    }

    fn hit_trees(&self, dx: usize, dy: usize) -> i64 {
        let mut x = 0;
        let mut y = 0;
        let mut hit_trees = 0;
        loop {
            match self.map.get(&(x, y)) {
                Some('#') => hit_trees += 1,
                Some('.') => {}
                _ => unreachable!(),
            }
            x += dx;
            x %= self.max_x + 1;
            y += dy;
            if y > self.max_y {
                break;
            }
        }
        hit_trees
    }
}
