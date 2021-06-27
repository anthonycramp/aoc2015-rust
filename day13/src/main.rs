use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("day13.txt");

fn main() {
    println!("Day 13 Part 1: {:?}", part1(INPUT));
    println!("Day 13 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let (names, happiness) = parse_seating_data(input);

    names
        .iter()
        .cloned()
        .permutations(names.len())
        .map(|seating| compute_happiness(&seating, &happiness))
        .max()
        .unwrap()
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let (mut names, mut happiness) = parse_seating_data(input);
    let me = String::from("me");

    for name in &names {
        happiness.insert((name.clone(), me.clone()), 0);
        happiness.insert((me.clone(), name.clone()), 0);
    }
    names.insert(me);

    names
        .iter()
        .cloned()
        .permutations(names.len())
        .map(|seating| compute_happiness(&seating, &happiness))
        .max()
        .unwrap()
}

fn parse(input: &str) -> ((String, String), i32) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(.*) would (.*) (.*) happiness units by sitting next to (.*).").unwrap();
    }

    let fields = RE.captures(input).unwrap();
    (
        (
            String::from(fields.get(1).unwrap().as_str()),
            String::from(fields.get(4).unwrap().as_str()),
        ),
        if let Ok(v) = fields.get(3).unwrap().as_str().parse() {
            if fields.get(2).unwrap().as_str() == "gain" {
                v
            } else {
                -v
            }
        } else {
            panic!("Couldn't parse happiness: {:?}", fields.get(3));
        },
    )
}

fn parse_seating_data(input: &str) -> (HashSet<String>, HashMap<(String, String), i32>) {
    let mut names: HashSet<String> = HashSet::new();
    let mut happiness: HashMap<(String, String), i32> = HashMap::new();

    for line in input.lines() {
        let ((name_a, name_b), happiness_gained) = parse(line);
        names.insert(name_a.clone());
        names.insert(name_b.clone());
        happiness.insert((name_a, name_b), happiness_gained);
    }

    (names, happiness)
}

/// Compute the happiness for a particular seating configuration
fn compute_happiness(seating: &[String], happiness: &HashMap<(String, String), i32>) -> i32 {
    let reverse_seating: Vec<String> = seating.iter().cloned().rev().collect();

    compute_happiness_in_order(seating, &happiness)
        + compute_happiness_in_order(&reverse_seating, &happiness)
}

fn compute_happiness_in_order(
    seating: &[String],
    happiness: &HashMap<(String, String), i32>,
) -> i32 {
    let mut growth_in_happiness = seating
        .windows(2)
        .map(|pair| happiness.get(&(pair[0].clone(), pair[1].clone())).unwrap())
        .sum();
    // add the happiness between the last seat and first seat
    growth_in_happiness += happiness
        .get(&(seating[seating.len() - 1].clone(), seating[0].clone()))
        .unwrap();

    growth_in_happiness
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_parse() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.";
        let ((name_a, name_b), happiness) = parse(input);

        assert_eq!(name_a, "Alice");
        assert_eq!(name_b, "Bob");
        assert_eq!(happiness, 54);

        let input = "Carol would lose 62 happiness units by sitting next to Alice.";
        let ((name_a, name_b), happiness) = parse(input);

        assert_eq!(name_a, "Carol");
        assert_eq!(name_b, "Alice");
        assert_eq!(happiness, -62);
    }
    #[test]
    fn test_part1() {
        let input = r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

        let change_in_happiness = part1(input);

        assert_eq!(change_in_happiness, 330);
    }
}
