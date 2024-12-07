use log::debug;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            solution.set(x, y, c);
        }
    }

    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    chairs: HashMap<(i64, i64), Position>,
    max_x: i64,
    max_y: i64,

    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
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
}

impl Solution {
    fn analyse_part1(&self) -> Option<i64> {
        let mut next = self.chairs.clone();
        let mut pass = 0;
        loop {
            pass += 1;
            debug!("P1 Pass {}", pass);
            let n = self.pass_part1(&next);
            next = n.0;
            let changed = n.1;
            Self::display(&next, self.max_x, self.max_y);
            if !changed {
                break;
            }
        }
        let occupied = Self::total_occupied(&next);
        Some(occupied)
    }

    fn analyse_part2(&self) -> Option<i64> {
        let mut next = self.chairs.clone();
        let mut pass = 0;
        loop {
            pass += 1;
            debug!("P2 Pass {}", pass);
            let n = self.pass_part2(&next);
            next = n.0;
            let changed = n.1;
            Self::display(&next, self.max_x, self.max_y);
            if !changed {
                break;
            }
        }
        let occupied = Self::total_occupied(&next);
        Some(occupied)
    }

    fn pass_part1(
        &self,
        current: &HashMap<(i64, i64), Position>,
    ) -> (HashMap<(i64, i64), Position>, bool) {
        let mut next = HashMap::new();
        let mut changed = false;
        for ((x, y), position) in current {
            let num_occupied = Self::num_occupied(current, *x, *y);
            let next_position = match position {
                Position::EmptySeat if num_occupied == 0 => {
                    changed = true;
                    Position::OccupiedSeat
                }
                Position::OccupiedSeat if num_occupied >= 4 => {
                    changed = true;
                    Position::EmptySeat
                }
                Position::EmptySeat => Position::EmptySeat,
                Position::OccupiedSeat => Position::OccupiedSeat,
                Position::Floor => Position::Floor,
            };
            next.insert((*x, *y), next_position);
        }
        (next, changed)
    }

    fn pass_part2(
        &self,
        current: &HashMap<(i64, i64), Position>,
    ) -> (HashMap<(i64, i64), Position>, bool) {
        let mut next = HashMap::new();
        let mut changed = false;
        for ((x, y), position) in current {
            let num_occupied_los = Self::num_occupied_los(current, *x, *y, self.max_x, self.max_y);
            let next_position = match position {
                Position::EmptySeat if num_occupied_los == 0 => {
                    changed = true;
                    Position::OccupiedSeat
                }
                Position::OccupiedSeat if num_occupied_los >= 5 => {
                    changed = true;
                    Position::EmptySeat
                }
                Position::EmptySeat => Position::EmptySeat,
                Position::OccupiedSeat => Position::OccupiedSeat,
                Position::Floor => Position::Floor,
            };
            next.insert((*x, *y), next_position);
        }
        (next, changed)
    }

    fn total_occupied(current: &HashMap<(i64, i64), Position>) -> i64 {
        current
            .iter()
            .filter(|((_x, _y), &position)| position == Position::OccupiedSeat)
            .count() as i64
    }

    fn display(current: &HashMap<(i64, i64), Position>, max_x: i64, max_y: i64) {
        for y in 0..=max_y {
            let mut line = String::new();
            for x in 0..=max_x {
                line += match current.get(&(x, y)).unwrap_or(&Position::Floor) {
                    Position::Floor => ".",
                    Position::EmptySeat => "L",
                    Position::OccupiedSeat => "#",
                };
            }
            debug!("{}", line);
        }
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        let m = match c {
            'L' => Position::EmptySeat,
            '.' => Position::Floor,
            '#' => Position::OccupiedSeat,
            _ => unreachable!(),
        };
        self.chairs.insert((x as i64, y as i64), m);
        self.max_x = max(self.max_x, x as i64);
        self.max_y = max(self.max_y, y as i64);
    }

    fn num_occupied(chairs: &HashMap<(i64, i64), Position>, x: i64, y: i64) -> i64 {
        let mut occupied = 0;
        for dy in -1..=1_i64 {
            for dx in -1..=1_i64 {
                if dx.abs() + dy.abs() == 0 {
                    continue;
                }
                if let Position::OccupiedSeat = chairs
                    .get(&(x + dx, y + dy))
                    .unwrap_or(&Position::EmptySeat)
                {
                    occupied += 1;
                }
            }
        }
        occupied
    }

    fn num_occupied_los(
        chairs: &HashMap<(i64, i64), Position>,
        x: i64,
        y: i64,
        max_x: i64,
        max_y: i64,
    ) -> i64 {
        let mut occupied = 0;
        for dy in -1..=1_i64 {
            for dx in -1..=1_i64 {
                if dx.abs() + dy.abs() == 0 {
                    continue;
                }
                let mut pos_x = x + dx;
                let mut pos_y = y + dy;
                let position = loop {
                    if pos_x < 0 || pos_y < 0 || pos_x > max_x || pos_y > max_y {
                        break Position::EmptySeat;
                    }
                    match chairs.get(&(pos_x, pos_y)) {
                        Some(Position::OccupiedSeat) => break Position::OccupiedSeat,
                        Some(Position::EmptySeat) => break Position::EmptySeat,
                        _ => {}
                    };
                    pos_x += dx;
                    pos_y += dy;
                };
                if let Position::OccupiedSeat = position {
                    occupied += 1;
                }
            }
        }
        occupied
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Position {
    EmptySeat,
    Floor,
    OccupiedSeat,
}
