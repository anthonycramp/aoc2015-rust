use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
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
fn part1(input: &str) -> usize {
    let machine = MoleculeMachine::from(input);
    machine.calibrate()
}

// replace return type as required by the problem
fn part2(input: &str) -> usize {
    let machine = MoleculeMachine::from(input);
    machine.reduce()
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
    fn generate_molecules(&self, start_molecule: &str) -> HashSet<String> {
        let mut molecules = HashSet::new();
        for (key, value) in &self.replacements {
            // split the starting molecule
            let fragments: Vec<String> = start_molecule.split(key).map(String::from).collect();

            // for each replacement
            for v in value.iter() {
                for i in 0..fragments.len() - 1 {
                    let mut replacement: Vec<String> = Vec::new();
                    for j in 0..fragments.len() - 1 {
                        if i == j {
                            replacement.push(v.clone());
                        } else {
                            replacement.push(key.clone());
                        }
                    }
                    let new_molecule: String = fragments
                        .iter()
                        .interleave(replacement.iter())
                        .cloned()
                        .collect();
                    molecules.insert(new_molecule);
                }
            }
        }

        molecules
    }

    fn calibrate(&self) -> usize {
        self.generate_molecules(&self.start).len()
    }

    fn synthesise(&self) -> usize {
        let mut steps = 0;
        let mut molecules = self.generate_molecules("e");

        loop {
            steps += 1;

            if molecules.contains(&self.start) {
                break;
            }

            for molecule in molecules.clone() {
                molecules.remove(&molecule);
                molecules.extend(self.generate_molecules(&molecule));
            }
        }
        steps
    }

    fn reduce(&self) -> usize {
        let mut steps;
        let mut reverse_replacements = HashMap::new();
        let mut rng = thread_rng();

        for (key, value) in &self.replacements {
            for v in value.iter() {
                reverse_replacements.insert(v, key);
            }
        }

        let mut fragments: Vec<&String> = reverse_replacements.keys().cloned().collect();

        loop {
            fragments.shuffle(&mut rng);
            let mut molecule = self.start.clone();
            steps = 0;
            loop {
                let mut found_replacement = false;
                for &fragment in fragments.iter() {
                    if molecule.find(fragment) != None {
                        molecule = molecule.replacen(fragment, reverse_replacements[fragment], 1);
                        found_replacement = true;
                        break;
                    }
                }

                if !found_replacement {
                    break;
                }

                steps += 1;
            }

            if molecule == "e" {
                break;
            }
        }
        steps
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
        assert_eq!(machine.calibrate(), 4);
    }

    #[test]
    fn test_replacement2() {
        let input = "H => HO
H => OH
O => HH

HOHOHO";
        let machine = MoleculeMachine::from(input);
        assert_eq!(machine.calibrate(), 7);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calibrate1() {
            let input = "H => HO
H => OH
O => HH

HOH";
            let machine = MoleculeMachine::from(input);
            assert_eq!(machine.calibrate(), 4);
        }

        #[test]
        fn test_calibrate2() {
            let input = "H => HO
H => OH
O => HH

HOHOHO";
            let machine = MoleculeMachine::from(input);
            assert_eq!(machine.calibrate(), 7);
        }

        #[test]
        fn test_synthesise1() {
            let input = "H => HO
H => OH
O => HH
e => H
e => O

HOH";
            let machine = MoleculeMachine::from(input);
            assert_eq!(machine.synthesise(), 3);
        }

        #[test]
        fn test_synthesise2() {
            let input = "H => HO
H => OH
O => HH
e => H
e => O

HOHOHO";
            let machine = MoleculeMachine::from(input);
            assert_eq!(machine.synthesise(), 6);
        }
    }

    #[test]
    fn test_reduce1() {
        let input = "H => HO
H => OH
O => HH
e => H
e => O

HOH";
        let machine = MoleculeMachine::from(input);
        assert_eq!(machine.reduce(), 3);
    }

    #[test]
    fn test_reduce2() {
        let input = "H => HO
H => OH
O => HH
e => H
e => O

HOHOHO";
        let machine = MoleculeMachine::from(input);
        assert_eq!(machine.reduce(), 6);
    }
}
