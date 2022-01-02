use log::debug;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::default();
    for line in reader.lines() {
        let line = line?;
        solution.add(Rule::from_str(&line)?);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    rules: Vec<Rule>,
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
}

impl Solution {
    pub fn analyse(&mut self) {
        self.answer_part1 = self.analyse_part1();
        self.answer_part2 = self.analyse_part2();
    }

    fn analyse_part1(&self) -> Option<i64> {
        let mut reverse_rules = HashMap::new();
        for rule in &self.rules {
            for (_count, target) in &rule.contains {
                reverse_rules
                    .entry(target.to_owned())
                    .or_insert_with(HashSet::new)
                    .insert(rule.source.clone());
            }
        }
        let mut pending = vec!["shiny gold".to_string()];
        let mut visited = HashSet::new();
        while let Some(bag) = pending.pop() {
            if visited.contains(&bag) {
                continue;
            }
            debug!("visited {}", bag);
            visited.insert(bag.clone());
            if let Some(other_bags) = reverse_rules.get(&bag) {
                for other_bag in other_bags {
                    if visited.contains(other_bag) {
                        continue;
                    }
                    pending.push(other_bag.to_owned());
                }
            }
        }
        // Remove 1 as shiny gold was visited first
        Some((visited.len() - 1) as i64)
    }

    fn analyse_part2(&self) -> Option<i64> {
        let rules_lookup = self.rules.iter().fold(HashMap::new(), |mut acc, rule| {
            acc.insert(rule.source.clone(), rule.contains.clone());
            acc
        });
        let mut pending = vec![(1, "shiny gold".to_string())];
        let mut total = 0;
        while let Some((count, bag)) = pending.pop() {
            debug!("P2 visited {} x {}", count, bag);
            total += count;
            if let Some(other_bags) = rules_lookup.get(&bag) {
                for (other_count, other_bag) in other_bags {
                    if other_bag == "no other" {
                        continue;
                    }
                    debug!("  {} {}", other_count, other_bag);
                    pending.push((count * other_count, other_bag.to_owned()));
                }
            }
        }
        // Remove 1 as shiny gold was visited first
        Some(total - 1)
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn add(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
}

#[derive(Debug, Default)]
struct Rule {
    source: String,
    contains: Vec<(i64, String)>,
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("{}", s);
        let r = Regex::new(r"^(?P<count>\d+)?(?P<color>.*) bags?\.?$").unwrap();
        let mut i = s.split(" bags contain ");
        let source = i.next().unwrap().to_string();
        debug!("source: {}", source);
        let contains = i
            .flat_map(|s| s.split(", "))
            .map(|s| {
                let c = r.captures(s).unwrap();
                let count = c
                    .name("count")
                    .map(|v| v.as_str().parse::<i64>().unwrap())
                    .unwrap_or(0);
                let color = c
                    .name("color")
                    .map(|v| v.as_str().trim().to_string())
                    .unwrap();
                (count, color)
            })
            .collect::<Vec<_>>();
        debug!("  {:?}", contains);
        Ok(Self { source, contains })
    }
}
