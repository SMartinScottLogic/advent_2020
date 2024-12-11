use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::default();
    for line in reader.lines().map_while(Result::ok) {
        if line.trim().is_empty() {
            continue;
        }
        match line.split_once(':') {
            Some(_) => solution.add_rule(line),
            None => solution.add_message(line),
        }
    }
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    rules: Vec<String>,
    messages: Vec<String>,
}

impl Solution {
    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Option<i64> {
        let rules = rules(self.rules.iter().cloned());
        let answer = self
            .messages
            .iter()
            .filter(|message| check_message(message.to_string(), &rules))
            .count();
        Some(answer as i64)
    }

    pub fn answer_part2(&self) -> Option<i64> {
        let mut part2_rules = self.rules.clone();
        part2_rules.push("8: 42 | 42 8".to_string());
        part2_rules.push("11: 42 31 | 42 11 31".to_string());
        let rules = rules(part2_rules.iter().cloned());
        let answer = self
            .messages
            .iter()
            .filter(|message| check_message(message.to_string(), &rules))
            .count();
        Some(answer as i64)
    }
}

/// Check an input line of text against the rule collection
pub fn check_message(message: String, rules: &HashMap<usize, RuleLogic>) -> bool {
    let solutions = process_rule(rules, 0, message, 0);
    // If any solution exists that has consumed the whole input, this is a pass
    solutions.contains(&String::from(""))
}

fn process_rule(
    rules: &HashMap<usize, RuleLogic>,
    index: usize,
    input: String,
    indent: usize,
) -> Vec<String> {
    let rule = &rules[&index];
    log::debug!(
        "{:-indent$}Checking: {:2}: {:?} Input: {}",
        "",
        index,
        rule,
        input,
        indent = indent,
    );
    let result: Vec<String> = match rule {
        // Match a single char
        RuleLogic::Simple(c) => simple_rule(*c, input).into_iter().collect(),
        // Or match a chain
        RuleLogic::Chain(indexes) => handle_chains(indexes, rules, input, indent),
    };
    log::debug!(
        "{:-indent$}{} {:?}",
        "",
        !result.is_empty(),
        result,
        indent = indent,
    );
    // Return whatever's left over from the input
    result
}

/// Handles a simple rule: does this character match with the next character
/// Returns the rest of the input if successful, or nothing if it fails
pub fn simple_rule(rule_char: char, input: String) -> Option<String> {
    input
        // For every unicode character (ie. decode utf-8)
        .chars()
        // Only take the first one
        .next()
        // Only take this first one if it matches the character in our rule
        .filter(|c2| *c2 == rule_char)
        // If it matches, return the rest of the input
        .map(|_| String::from(&input[1..]))
}

// Takes a bunch of alternate rule index chains, We must try each possibility
fn handle_chains(
    chains: &[Vec<usize>],
    rules: &HashMap<usize, RuleLogic>,
    input: String,
    indent: usize,
) -> Vec<String> {
    // Collect all the possibilites, and use the one that consumes the most
    chains
        // Look at every posssible chain
        .iter()
        // Only take chains that pass
        .flat_map(|this_chain| chain(this_chain, rules, input.clone(), indent + 1))
        .collect()
}

/// Takes a chain of rule indexes, if they all match, it returns the rest of the string
/// If any fail, it returns an empty vec
fn chain(
    chain: &[usize],
    rules: &HashMap<usize, RuleLogic>,
    input: String,
    indent: usize,
) -> Vec<String> {
    chain
        .iter()
        // Try to go through all the links in the chain
        .try_fold(vec![input], |solutions, index| {
            // For each previous output, reuse it as an input to process this link in the chain
            let new_solutions: Vec<String> = solutions
                .iter()
                // Find all the possibilites that match using each of the
                // previous inputs, and the next rule index in the chain,
                // then flatten them into the possible output solutions (which
                // will be used for input to the next link in the chain, or for
                // the last link, returned)
                .flat_map(|input| process_rule(rules, *index, input.clone(), indent))
                .collect();
            if new_solutions.is_empty() {
                // If the next link, using the previous output as input, found no solutions
                // The chain is broken
                None
            } else {
                // Continue to the next link, providing all the
                // solutions/remainders we've found so far as its input
                Some(new_solutions)
            }
        })
        .unwrap_or_else(Vec::new)
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub number: usize,
    pub logic: RuleLogic,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuleLogic {
    // Looks like: "a"
    // Input must be 'a' for this to pass
    Simple(char),
    // Looks like: 1 3 | 3 1
    // Input must match rule 1, then the next char rule 3 ... or ... the other way around
    Chain(Vec<Vec<usize>>),
}

fn rule_number(input: &str) -> IResult<&str, usize> {
    terminated(number, tag(": "))(input)
}

fn simple_char(input: &str) -> IResult<&str, char> {
    delimited(tag("\""), anychar, tag("\""))(input)
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |digits: &str| digits.parse::<usize>())(input)
}

fn simple_chain(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(" "), number)(input)
}

fn chains(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(tag(" | "), simple_chain)(input)
}

pub fn rule(input: &str) -> IResult<&str, Rule> {
    let char = simple_char.map(RuleLogic::Simple);
    let chains = chains.map(RuleLogic::Chain);
    let rule_logic = alt((char, chains));
    let (rest, (number, logic)) = pair(rule_number, rule_logic)(input)?;
    Ok((rest, Rule { number, logic }))
}

/// Parses a bunch of rules and returns their logic in order
pub fn rules(lines: impl Iterator<Item = String>) -> HashMap<usize, RuleLogic> {
    lines
        .map(|line| rule(line.as_str()).map(|(_rest, rule)| rule).unwrap())
        .map(|rule| (rule.number, rule.logic))
        .collect()
}
