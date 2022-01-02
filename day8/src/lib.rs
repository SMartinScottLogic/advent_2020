use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;
use log::debug;
use std::collections::HashMap;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);

    let mut solution = Solution::default();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        solution.add_operation(Operation::from_str(line)?);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    program: Vec<Operation>,
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
}

impl Solution {

    pub fn analyse(&mut self) {
        self.answer_part1 = Some(self.analyse_part1());
        self.answer_part2 = Some(self.analyse_part2());
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn add_operation(&mut self, operation: Operation) {
        self.program.push(operation);
    }

    fn analyse_part1(&self) -> i64 {
        use Operation::*;

        let mut pc = 0_isize;
        let mut accumulator = 0;
        let mut count = HashMap::new();
        loop {
            debug!("{} {:?} {}", pc, self.program[pc as usize], accumulator);
            *count.entry(pc).or_insert(0) += 1;
            if count[&pc] > 1 {
                return accumulator;
            }
            match self.program[pc as usize] {
                Acc(v) => accumulator += v as i64,
                Jmp(v) => pc += v - 1,
                Nop(_v) => {},
            };
            pc += 1;
        }
    }

    fn analyse_part2(&self) -> i64 {
        use Operation::*;

        for (id, _operation) in self.program.iter().enumerate() {
            let mut program = self.program.clone();
            program[id] = match program[id] {
                Acc(v) => Acc(v),
                Jmp(v) => Nop(v),
                Nop(v) => Jmp(v),
            };
            let mut pc = 0_isize;
            let mut accumulator = 0;
            let mut count = HashMap::new();
            'inner: loop {
            if pc == program.len() as isize {
                return accumulator;
            }
            debug!("{} {:?} {}", pc, program[pc as usize], accumulator);
            *count.entry(pc).or_insert(0) += 1;
            if count[&pc] > 1 {
                break 'inner;
            }
            match program[pc as usize] {
                Acc(v) => accumulator += v as i64,
                Jmp(v) => pc += v - 1,
                Nop(_v) => {},
            };
            pc += 1;
        }
        };
        -1
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
enum Operation {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("s: {}", s);
        let r = regex::Regex::new(r"^(?P<op>[^\s]+) (?P<arg>[+-]\d+)$").unwrap();
        let c = r.captures(s).unwrap();
        let op = c.name("op").unwrap().as_str().to_string();
        let arg = c.name("arg").unwrap().as_str().parse().unwrap();
        let operation = match c.name("op").map(|v| v.as_str()) {
            Some("acc") => Self::Acc(arg),
            Some("jmp") => Self::Jmp(arg),
            Some("nop") => Self::Nop(arg),
            _ => unreachable!()
        };
        Ok(operation)
    }
}