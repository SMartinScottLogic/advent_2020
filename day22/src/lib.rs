use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use log::{debug, info};
use memoize::memoize;

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
            debug!("pot: {:?}", pot);
            let winner = pot[0].0;
            for (_, card) in pot {
                hands.entry(winner).or_default().push_back(card);
            }
        };
        info!("hands: {:?}", hands);
        let winner = hands.into_iter().find(|(_, v)| !v.is_empty())
        .unwrap().1;
        let r = winner.iter().rev().enumerate().map(|(i, v)| (i+1) as i64 * *v).sum();
        Some(r)
    }

    pub fn answer_part2(&self) -> Option<i64> {
        let mut hands = Vec::new();
        hands.resize(3, VecDeque::new());
        for (id, hand) in &self.hands {
            hands[*id as usize] = hand.clone();
        }
        let (winner, hands) = play_recursive(hands.clone(), 0);
        info!("hands: {:?}", hands);
        let r = hands.get(winner).unwrap().iter().rev().enumerate().map(|(i, v)| (i+1) as i64 * *v).sum();
        Some(r)
    }
}

#[memoize]
fn play_recursive(mut hands: Vec<VecDeque<i64>>, level: usize) -> (usize, Vec<VecDeque<i64>>) {
    let original_hands = hands.clone();
    let mut history = Vec::new();
    let (winner, hands) = loop {
        let mut not_empty = HashSet::new();
        for (k, v) in hands.iter().enumerate() {
            if level == 0 {
            info!("Player {}'s deck: {:?}", k, v);
            }
            if !v.is_empty() {
                not_empty.insert(k);
            }
        }
        if not_empty.len() == 1 {
            let winner = *not_empty.iter().next().unwrap();
            break (winner, hands.clone());
        }

        if history.contains(&hands) {
            break (1, hands.clone());
        }
        history.push(hands.clone());

        let mut pot = Vec::new();
        for (k, v) in hands.iter_mut().enumerate() {
            if !v.is_empty() {
            let card = v.pop_front().unwrap();
            pot.push((k, card));
            }
        }
        pot.sort_by_key(|v| -v.1);
        debug!("pot: {:?}", pot);
        // Check for Recursive
        let mut recurse = true;
        for (k, v) in &pot {
            if (hands[*k].len() as i64) < *v {
                recurse = false;
            }
        }
        let winner = if recurse {
            if level == 0 {
            info!("launch recursive game from {:?}", hands);
            }
            play_recursive(hands.clone(), level+1).0
        } else {
            pot[0].0
        };
        pot.sort_by_key(|v| winner.abs_diff(v.0));
            for (_, card) in pot {
                hands.get_mut(winner).unwrap().push_back(card);
            }
    };
    (winner, hands)
}