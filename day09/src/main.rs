use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("day09.txt");

fn main() {
    println!("Day 09 Part 1: {:?}", part1(INPUT));
    println!("Day 09 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let (routes, places) = parse_routes(input);
    places
        .iter()
        .cloned()
        .permutations(places.len())
        .map(|place_seq| compute_distance(&place_seq, &routes))
        .min()
        .unwrap()
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

fn parse_route(input: &str) -> ((String, String), i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.*) to (.*) = (.*)").unwrap();
    }

    let fields = RE.captures(input).unwrap();
    (
        (
            String::from(fields.get(1).unwrap().as_str()),
            String::from(fields.get(2).unwrap().as_str()),
        ),
        fields.get(3).unwrap().as_str().parse().unwrap(),
    )
}

fn parse_routes(input: &str) -> (HashMap<(String, String), i32>, HashSet<String>) {
    let mut routes = HashMap::new();
    let mut places = HashSet::new();

    for line in input.lines() {
        let ((place_a, place_b), dist) = parse_route(line);
        routes.insert((place_a.clone(), place_b.clone()), dist);
        routes.insert((place_b.clone(), place_a.clone()), dist);
        places.insert(place_a);
        places.insert(place_b);
    }

    (routes, places)
}

fn compute_distance(place_seq: &[String], routes: &HashMap<(String, String), i32>) -> i32 {
    place_seq
        .windows(2)
        .map(|pair| routes.get(&(pair[0].clone(), pair[1].clone())).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let input = r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

        assert_eq!(part1(input), 605);
    }

    #[test]
    fn test_part2() {
        let input = r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

        assert_eq!(part2(input), 605);
    }
}
