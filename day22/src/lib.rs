use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use log::info;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::new();
    let mut player = 0;
    for line in reader.lines().map_while(Result::ok) {
        if line.trim().is_empty() {
            continue;
        }
        if line.contains("Player") {
            player += 1;
            continue;
        }
        solution.add_card(player, line.parse().unwrap());
    }


    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    hands: HashMap<i32, VecDeque<i64>>,
}

impl Solution {
    fn new() -> Self {
        Self::default()
    }

    fn add_card(&mut self, player: i32, card: i64) {
        self.hands.entry(player).or_default().push_back(card);
    }

    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Option<i64> {
        let mut hands = self.hands.clone();
        'out:loop {
            for (k, v) in &hands {
                if v.is_empty() {
                    break 'out;
                }
            }
            let mut pot = Vec::new();
            for (k, v) in hands.iter_mut() {
                let card = v.pop_front().unwrap();
                pot.push((*k, card));
            }
            pot.sort_by_key(|v| -v.1);
            info!("pot: {:?}", pot);
            let winner = pot[0].0;
            for (_, card) in pot {
                hands.entry(winner).or_default().push_back(card);
            }
        };
        info!("hands: {:?}", hands);
        let winner = hands.into_iter()
        .filter(|(k, v)| v.len() > 0)
        .next()
        .unwrap().1;
        let r = winner.iter().rev().enumerate().map(|(i, v)| (i+1) as i64 * *v).sum();
        Some(r)
    }

    pub fn answer_part2(&self) -> Option<i64> {
        None
    }
}
