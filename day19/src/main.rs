use regex::Regex;

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("day19.txt");

fn main() {
    println!("Day 19 Part 1: {:?}", part1(INPUT));
    println!("Day 19 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    0
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

struct MoleculeMachine {
    replacements: HashMap<String, Vec<String>>,
    start: String,
}

impl From<&str> for MoleculeMachine {
    fn from(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*) => (.*)").unwrap();
        }
        let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
        let mut start = String::new();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(fields) = RE.captures(line) {
                replacements
                    .entry(String::from(fields.get(1).unwrap().as_str()))
                    .or_default()
                    .push(String::from(fields.get(2).unwrap().as_str()));
            } else {
                start = String::from(line);
            }
        }
        Self {
            replacements,
            start,
        }
    }
}

impl MoleculeMachine {
    fn generate_molecules(&self) -> HashSet<String> {
        let molecules = HashSet::new();

        molecules
    }

    fn count_molecules(&self) -> usize {
        self.generate_molecules().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replacement1() {
        let input = "H => HO
H => OH
O => HH

HOH";
        let machine = MoleculeMachine::from(input);
        assert_eq!(machine.count_molecules(), 4);
    }

    #[test]
    fn test_replacement2() {
        let input = "H => HO
H => OH
O => HH

HOHOHO";
        let machine = MoleculeMachine::from(input);
        assert_eq!(machine.count_molecules(), 7);
    }
}
