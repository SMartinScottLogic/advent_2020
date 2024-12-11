use log::debug;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut solution = Solution::default();
    let mut mode = ParseState::Initial;
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match line {
            "your ticket:" => mode = ParseState::MyTicket,
            "nearby tickets:" => mode = ParseState::NearTickets,
            _ => match mode {
                ParseState::Initial => {
                    solution.add_rule(Rule::from_str(line)?);
                }
                ParseState::MyTicket => {
                    solution.set_my_ticket(Ticket::from_str(line)?);
                }
                ParseState::NearTickets => {
                    solution.add_near_ticket(Ticket::from_str(line)?);
                }
            },
        }
    }
    Ok(solution)
}

#[derive(Debug)]
enum ParseState {
    Initial,
    MyTicket,
    NearTickets,
}

#[derive(Debug, Default)]
pub struct Solution {
    my_ticket: Ticket,
    near_tickets: Vec<Ticket>,
    rules: Vec<Rule>,
}

impl Solution {
    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Option<i64> {
        let mut invalid_values = Vec::new();
        for ticket in &self.near_tickets {
            for value in &ticket.values {
                if !self.valid_ticket_value(*value) {
                    invalid_values.push(*value);
                }
            }
        }
        debug!("Invalid values: {:?}", invalid_values.len());
        Some(invalid_values.iter().sum())
    }

    pub fn answer_part2(&self) -> Option<i64> {
        let mut count = 0;
        let mut categories = Vec::new();
        for ticket in self
            .near_tickets
            .iter()
            .filter(|ticket| ticket.values.iter().all(|v| self.valid_ticket_value(*v)))
        {
            let t = ticket
                .values
                .iter()
                .map(|value| {
                    let mut rules = HashSet::new();
                    for rule in &self.rules {
                        for (start, end) in &rule.ranges {
                            if value >= start && value <= end {
                                rules.insert(rule.name.clone());
                            }
                        }
                    }
                    rules
                })
                .collect::<Vec<_>>();

            debug!("P2 t {:#?}", t);

            count += 1;
            if count == 1 {
                categories = t;
            } else {
                categories.iter_mut().enumerate().for_each(|(idx, v)| {
                    let other = t.get(idx).map(|v| v.to_owned()).unwrap_or_default();
                    let a = v
                        .intersection(&other)
                        .map(|v| v.to_owned())
                        .collect::<HashSet<_>>();
                    debug!(
                        "P2 {}: intersection of {:?} and {:?} = {:?}",
                        idx, v, other, a
                    );
                    v.retain(|v| a.contains(v));
                });
            }
        }
        let mut known = HashMap::new();
        loop {
            debug!("{:#?}", categories);
            let mut change = false;
            {
                for (idx, category) in categories.iter().enumerate() {
                    if category.len() == 1
                        && known
                            .insert(category.iter().next().unwrap().to_owned(), idx)
                            .is_none()
                    {
                        change = true;
                    }
                }
            }
            let next_categories = categories
                .iter()
                .enumerate()
                .map(|(idx, cats)| {
                    let cats = cats
                        .iter()
                        .filter(|c| !known.contains_key(*c) || known.get(*c).unwrap() == &idx)
                        .map(|c| c.to_owned())
                        .collect::<HashSet<_>>();
                    (idx, cats)
                })
                .map(|(_idx, v)| v)
                .collect::<Vec<_>>();
            categories = next_categories;
            if !change {
                break;
            }
        }
        debug!("{:?}", known);
        debug!("{} / {}", count, self.near_tickets.len());
        let mut total = 1_i64;
        for (k, idx) in known
            .into_iter()
            .filter(|(k, _v)| k.starts_with("departure"))
        {
            debug!(
                "{}, {} = {}",
                k,
                idx,
                self.my_ticket.values.get(idx).unwrap()
            );
            total *= self.my_ticket.values.get(idx).unwrap();
        }
        Some(total)
    }
}

impl Solution {
    fn add_near_ticket(&mut self, ticket: Ticket) {
        self.near_tickets.push(ticket);
    }

    fn set_my_ticket(&mut self, ticket: Ticket) {
        self.my_ticket = ticket;
    }

    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn valid_ticket_value(&self, value: i64) -> bool {
        for rule in &self.rules {
            for (start, end) in &rule.ranges {
                if value >= *start && value <= *end {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Default)]
struct Ticket {
    values: Vec<i64>,
}

impl FromStr for Ticket {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.trim().split(',').map(|v| v.parse().unwrap()).collect();

        Ok(Self { values })
    }
}

#[derive(Debug, Default)]
struct Rule {
    name: String,
    ranges: Vec<(i64, i64)>,
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s.trim().find(':').unwrap();
        let name = (s[..pos]).to_string();

        let ranges = (s[pos + 1..])
            .trim()
            .split(" or ")
            .map(|v| {
                v.split_once("-")
                    .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
                    .unwrap()
            })
            .collect();

        Ok(Self { name, ranges })
    }
}
