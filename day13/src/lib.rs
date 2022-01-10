use std::fs::File;
use std::io::{BufRead, BufReader};
use log::debug;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);
    let mut solution = Solution::new();

    let mut line = String::new();
    reader.read_line(&mut line)?;
    solution.set_earliest_departure(line.trim().parse()?);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    debug!("'{}'", line);
    line.trim().split(',').map(|s| s.parse::<i64>().ok()).for_each(|v| solution.add_bus(v));

    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
    
    earliest_departure: i64,
    buses: Vec<Option<i64>>,
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

    pub fn set_earliest_departure(&mut self, time: i64) {
        self.earliest_departure = time;
    }

    pub fn add_bus(&mut self, bus: Option<i64>) {
        self.buses.push(bus);
    }

    fn analyse_part1(&self) -> Option<i64> {
        let mut best = None;
        for bus in self.buses.iter().flatten() {
            let departure = (self.earliest_departure / bus) + 1;
            let departure = departure * bus;
            let wait = departure - self.earliest_departure;
            debug!("{} {} vs {}", bus, wait, self.earliest_departure);
            match best {
                    Some((_b, w)) if w <= wait => {},
                    _ => best = Some((*bus, wait)),
            }
        }
        debug!("{:?}", best);
        best.map(|(bus, wait)| bus * wait)
    }

    fn analyse_part2(&self) -> Option<i64> {
        None
    }
}
