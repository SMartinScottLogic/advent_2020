use itertools::Itertools;
use log::debug;
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
        solution.add(Sum::from_str(&line)?);
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    sums: Vec<Sum>,
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
}

impl Solution {
    pub fn analyse(&mut self) {
        let total_part1 = self.sums.iter().map(|sum| sum.evaluate_part1()).sum();
        let total_part2 = self.sums.iter().map(|sum| sum.evaluate_part2()).sum();

        self.answer_part1 = Some(total_part1);
        self.answer_part2 = Some(total_part2);
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn add(&mut self, sum: Sum) {
        self.sums.push(sum);
    }
}

#[derive(Debug, Default)]
struct Sum {
    s: String,
}

#[derive(Debug, Clone, Copy)]
enum AlgorithmicObject {
    Value(i64),
    Add,
    Mul,
}

#[derive(Debug, Default)]
struct CalculationPart1 {
    calc: Vec<AlgorithmicObject>,
}

impl CalculationPart1 {
    fn new() -> Self {
        Self::default()
    }

    fn value(&self) -> Option<i64> {
        match self.calc.len() {
            1 => self.calc.first().and_then(|v| match v {
                AlgorithmicObject::Value(v) => Some(*v),
                _ => None,
            }),
            _ => None,
        }
    }

    fn push(&mut self, value: AlgorithmicObject) {
        use AlgorithmicObject::*;

        self.calc.push(value);
        if self.calc.len() == 3 {
            let triple = self.calc.iter().collect_tuple();
            let r = match triple {
                Some((Value(lhs), Add, Value(rhs))) => Value(lhs + rhs),
                Some((Value(lhs), Mul, Value(rhs))) => Value(lhs * rhs),
                _ => unreachable!(),
            };
            self.calc.clear();
            self.calc.push(r);
        }
    }
}

#[derive(Debug, Default)]
struct CalculationPart2 {
    calc: Vec<AlgorithmicObject>,
}

impl CalculationPart2 {
    fn new() -> Self {
        Self::default()
    }

    fn value(&self) -> Option<i64> {
        use AlgorithmicObject::*;

        let mut next = Vec::new();
        let mut prev2 = None;
        let mut prev = None;
        // 1. Perform addition
        for v in self.calc.iter() {
            match (prev2, prev, v) {
                (Some(Value(lhs)), Some(Add), Value(rhs)) => {
                    prev2 = None;
                    prev = Some(Value(lhs + rhs));
                }
                _ => {
                    if let Some(v) = prev2 {
                        next.push(v);
                    }
                    prev2 = prev;
                    prev = Some(*v);
                }
            }
        }
        if let Some(v) = prev2 {
            next.push(v);
        }
        if let Some(v) = prev {
            next.push(v);
        }
        debug!("next: {:?}", next);
        let calc = next;
        // 2. Perform multiplication
        let mut next = Vec::new();
        let mut prev2 = None;
        let mut prev = None;
        for v in calc.iter() {
            match (prev2, prev, v) {
                (Some(Value(lhs)), Some(Mul), Value(rhs)) => {
                    prev2 = None;
                    prev = Some(Value(lhs * rhs));
                }
                _ => {
                    if let Some(v) = prev2 {
                        next.push(v);
                    }
                    prev2 = prev;
                    prev = Some(*v);
                }
            }
        }
        if let Some(v) = prev2 {
            next.push(v);
        }
        if let Some(v) = prev {
            next.push(v);
        }
        debug!("next: {:?}", next);
        let calc = next;
        match calc.len() {
            1 => calc.first().and_then(|v| match v {
                AlgorithmicObject::Value(v) => Some(*v),
                _ => None,
            }),
            _ => None,
        }
    }

    fn push(&mut self, value: AlgorithmicObject) {
        self.calc.push(value);
    }
}

impl Sum {
    fn evaluate_part1_part(s: &[u8]) -> (i64, usize) {
        use AlgorithmicObject::*;

        let mut cur_val = None;
        let mut stack = CalculationPart1::new();
        let mut idx = 0;
        while idx < s.len() {
            let c = s[idx];
            match c {
                b' ' => {
                    if let Some(v) = cur_val {
                        stack.push(Value(v));
                        cur_val = None;
                    }
                }
                b'0'..=b'9' => {
                    let next_val = match cur_val {
                        None => (c as i8 - 0x30) as i64,
                        Some(v) => v + (c as i8 - 0x30) as i64,
                    };
                    cur_val = Some(next_val);
                }
                b'+' => {
                    stack.push(Add);
                }
                b'*' => {
                    stack.push(Mul);
                }
                b'(' => {
                    debug!("inner");
                    let (result, mut shift) = Self::evaluate_part1_part(&s[idx + 1..]);
                    shift += 1;
                    debug!(
                        "inner shift idx from {} to {} by {} (next = {:?})",
                        idx,
                        idx + shift,
                        shift,
                        s.get(idx + shift)
                    );
                    idx += shift;
                    stack.push(Value(result));
                }
                b')' => {
                    if let Some(v) = cur_val {
                        stack.push(Value(v));
                    }
                    debug!("return from inner ({:?})", stack);
                    return (stack.value().unwrap(), idx);
                }
                _ => unreachable!(),
            };
            debug!("stack:= {:?}", stack);
            idx += 1;
        }
        (stack.value().unwrap(), idx)
    }

    fn evaluate_part2_part(s: &[u8]) -> (i64, usize) {
        use AlgorithmicObject::*;

        let mut cur_val = None;
        let mut stack = CalculationPart2::new();
        let mut idx = 0;
        while idx < s.len() {
            let c = s[idx];
            match c {
                b' ' => {
                    if let Some(v) = cur_val {
                        stack.push(Value(v));
                        cur_val = None;
                    }
                }
                b'0'..=b'9' => {
                    let next_val = match cur_val {
                        None => (c as i8 - 0x30) as i64,
                        Some(v) => v + (c as i8 - 0x30) as i64,
                    };
                    cur_val = Some(next_val);
                }
                b'+' => {
                    stack.push(Add);
                }
                b'*' => {
                    stack.push(Mul);
                }
                b'(' => {
                    debug!("inner");
                    let (result, mut shift) = Self::evaluate_part2_part(&s[idx + 1..]);
                    shift += 1;
                    debug!(
                        "inner shift idx from {} to {} by {} (next = {:?})",
                        idx,
                        idx + shift,
                        shift,
                        s.get(idx + shift)
                    );
                    idx += shift;
                    stack.push(Value(result));
                }
                b')' => {
                    if let Some(v) = cur_val {
                        stack.push(Value(v));
                    }
                    debug!("return from inner ({:?})", stack);
                    return (stack.value().unwrap(), idx);
                }
                _ => unreachable!(),
            };
            debug!("stack:= {:?}", stack);
            idx += 1;
        }
        (stack.value().unwrap(), idx)
    }

    fn evaluate_part1(&self) -> i64 {
        let s = self.s.clone() + " ";
        let s = s.as_bytes();
        let (r, _idx) = Self::evaluate_part1_part(s);
        debug!("{} => {:?}", self.s.len(), r);
        r
    }

    fn evaluate_part2(&self) -> i64 {
        let s = self.s.clone() + " ";
        let s = s.as_bytes();
        let (r, _idx) = Self::evaluate_part2_part(s);
        debug!("{} => {:?}", self.s.len(), r);
        r
    }
}

impl FromStr for Sum {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { s: s.to_string() })
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_part2(sum: &str, expected: i64) {
        let sum = Sum::from_str(sum).unwrap();
        assert_eq!(sum.evaluate_part2(), expected);
    }

    #[test]
    fn test_values_1() {
        test_part2("1 + (2 * 3) + (4 * (5 + 6))", 51);
    }

    #[test]
    fn test_values_2() {
        test_part2("2 * 3 + (4 * 5)", 46);
    }

    #[test]
    fn test_values_3() {
        test_part2("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)
    }

    #[test]
    fn test_value_3i() {
        test_part2("8 * 3 + 9 + 3 * 4 * 3", 1440)
    }

    #[test]
    fn test_values_4() {
        test_part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060);
    }

    #[test]
    fn test_values_5() {
        test_part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340);
    }
}
