use std::collections::HashSet;

const INPUT: &'static str = include_str!("inputs/day03.txt");

fn main() {
    println!("Day 03 Part 1: {}", part1(INPUT));
}

fn part1(input: &str) -> u32 {
    get_houses_delivered(input).len() as u32
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Location {
    x: i32,
    y: i32,
}

fn get_houses_delivered(input: &str) -> HashSet<Location> {
    let mut houses_delivered = HashSet::new();

    let mut current_house = Location { x: 0, y: 0 };
    houses_delivered.insert(current_house.clone());
    for c in input.chars() {
        match c {
            '^' => current_house.y += 1,
            'v' => current_house.y -= 1,
            '<' => current_house.x -= 1,
            '>' => current_house.x += 1,
            _ => (),
        }
        houses_delivered.insert(current_house.clone());
    }

    houses_delivered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        struct TestCase {
            input: &'static str,
            expected_output: u32,
        }

        let test_cases = [
            TestCase {
                input: ">",
                expected_output: 2,
            },
            TestCase {
                input: "^>v<",
                expected_output: 4,
            },
            TestCase {
                input: "^v^v^v^v^v",
                expected_output: 2,
            },
        ];

        for TestCase {
            input,
            expected_output,
        } in test_cases.iter()
        {
            assert_eq!(get_houses_delivered(*input).len() as u32, *expected_output);
        }
    }
}