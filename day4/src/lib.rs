use anyhow::Context;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[macro_use]
extern crate lazy_static;
use log::{debug, error};

lazy_static! {
    static ref REQUIRED_PASSPORT_FIELDS: HashMap<&'static str, regex::Regex> = {
        let mut m = HashMap::new();
        m.insert("byr", regex::Regex::new(r"^(?P<value>[0-9]{4})$").unwrap());
        m.insert("iyr", regex::Regex::new(r"^(?P<value>[0-9]{4})$").unwrap());
        m.insert("eyr", regex::Regex::new(r"^(?P<value>[0-9]{4})$").unwrap());
        m.insert("hgt", regex::Regex::new(r"^(?P<value>[0-9]{2,3})(?P<unit>cm|in)$").unwrap());
        m.insert("hcl", regex::Regex::new(r"^#(?P<value>[0-9a-f]{6})$").unwrap());
        m.insert("ecl", regex::Regex::new(r"^(?P<value>amb|blu|brn|gry|grn|hzl|oth)$").unwrap());
        m.insert("pid", regex::Regex::new(r"^(?P<value>[0-9]{9})$").unwrap());
        m
};
/*
    static ref REQUIRED_PASSPORT_FIELDS: Vec<(&'static str, regex::Regex)> =
        vec![
            ("byr", regex::Regex::new(r"^(?P<value>[0-9]{4})$").unwrap()),
            ("iyr", regex::Regex::new(r"^(?P<value>[0-9]{4})$").unwrap()),
            ("eyr", regex::Regex::new(r"^(?P<value>[0-9]{4})$").unwrap()),
            ("hgt", regex::Regex::new(r"^(?P<value>[0-9]{2-3})(?P<unit>cm|in)$").unwrap()),
            ("hcl", regex::Regex::new(r"^#(?P<value>[0-9a-f]{6})$").unwrap()),
            ("ecl", regex::Regex::new(r"^(?P<value>amb|blu|brn|gry|grn|hzl|oth)$").unwrap()),
            ("pid", regex::Regex::new(r"^(?P<value>[0-9]{9})$").unwrap()),
        //"cid"
        ];
        */
}

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename).context(format!("loading '{}'", filename))?;

    let mut solution = Solution::default();
    let reader = BufReader::new(file);
    let mut passport = String::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            solution.add_passport(&passport);
            passport = String::new();
        } else {
            passport.push(' ');
            passport.push_str(line);
        }
    }
    solution.add_passport(&passport);
    Ok(solution)
}

#[derive(Debug, Default)]
pub struct Solution {
    passports: Vec<HashMap<String, String>>,
    answer_part1: Option<i64>,
    answer_part2: Option<i64>,
}

impl Solution {
    pub fn analyse(&mut self) {
        let num_valid_part1 = self
            .passports
            .iter()
            .filter(|passport| !self.is_invalid_part1(passport))
            .count();
        let num_valid_part2 = self
            .passports
            .iter()
            .filter(|passport| !self.is_invalid_part1(passport))
            .filter(|passport| !Self::is_invalid_part2(passport))
            .count();
        self.answer_part1 = Some(num_valid_part1 as i64);
        self.answer_part2 = Some(num_valid_part2 as i64);
    }

    pub fn answer_part1(&self) -> Option<i64> {
        self.answer_part1
    }

    pub fn answer_part2(&self) -> Option<i64> {
        self.answer_part2
    }
}

impl Solution {
    fn add_passport(&mut self, passport: &str) {
        let passport = passport
            .trim()
            .split_whitespace()
            .map(|s| {
                let pos = s.find(':').unwrap();
                let first = s[..pos].to_string();
                let rest = s[(pos + 1)..].to_string();
                (first, rest)
            })
            .collect::<HashMap<_, _>>();
        self.passports.push(passport);
    }

    fn is_invalid_part1(&self, passport: &HashMap<String, String>) -> bool {
        REQUIRED_PASSPORT_FIELDS
            .keys()
            .any(|field| !passport.contains_key(*field))
    }

    fn is_invalid_part2(passport: &HashMap<String, String>) -> bool {
        for (k, v) in passport {
            if let Some(r) = REQUIRED_PASSPORT_FIELDS.get(&k[..]) {
                match r.captures(&v[..]) {
                    None => {
                        error!("{} {}", k, v);
                        return true;
                    }
                    Some(c) => match c.name("value").map(|v| v.as_str()) {
                        None => {
                            error!("{} {}", k, v);
                            return true;
                        }
                        Some(value) => {
                            debug!("{} {} {}", k, v, value);
                            if match &k[..] {
                                "byr" => {
                                    matches!(value.parse::<i64>(), Ok(value) if !(1920..=2002).contains(&value))
                                }
                                "iyr" => {
                                    matches!(value.parse::<i64>(), Ok(value) if !(2010..=2020).contains(&value))
                                }
                                "eyr" => {
                                    matches!(value.parse::<i64>(), Ok(value) if !(2020..=2030).contains(&value))
                                }
                                "hgt" => {
                                    let value = value.parse::<i64>();
                                    let units = c.name("unit").map(|v| v.as_str()).unwrap();
                                    matches!(value, Ok(value) if units=="cm" && !(150..=193).contains(&value))
                                        || matches!(value, Ok(value) if units=="in" && !(59..=76).contains(&value))
                                }
                                "hcl" => false,
                                "ecl" => false,
                                "pid" => false,
                                _ => false,
                            } {
                                return true;
                            }
                        }
                    },
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_values() {
        for (key, value, expected) in [
            ("byr", "2002", false),
            ("byr", "2003", true),
            ("hgt", "60in", false),
            ("hgt", "190cm", false),
            ("hgt", "190in", true),
            ("hgt", "190", true),
            ("hcl", "#123abc", false),
            ("hcl", "#123abz", true),
            ("hcl", "123abc", true),
            ("ecl", "brn", false),
            ("ecl", "wat", true),
            ("pid", "000000001", false),
            ("pid", "0123456789", true),
        ] {
            let mut passport = HashMap::new();
            passport.insert(key.to_string(), value.to_string());
            let result = Solution::is_invalid_part2(&passport);
            assert_eq!(result, expected);
        }
    }
}
