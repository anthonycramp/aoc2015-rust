use regex::Regex;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("day13.txt");

fn main() {
    println!("Day 13 Part 1: {:?}", part1(INPUT));
    println!("Day 13 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    0
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_parse() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.";
        let parsed = parse(input);

        assert_eq!(parsed.0 .0, "Alice");
        assert_eq!(parsed.0 .1, "Bob");
        assert_eq!(parsed.1, 54);

        let input = "Carol would lose 62 happiness units by sitting next to Alice.";
        let parsed = parse(input);

        assert_eq!(parsed.0 .0, "Carol");
        assert_eq!(parsed.0 .1, "Alice");
        assert_eq!(parsed.1, -62);
    }
    #[test]
    fn test_part1() {
        let test_cases = [
            TestCase {
                input: "...",
                expected: 123,
            },
            TestCase {
                input: "abc",
                expected: 345,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [
            TestCase {
                input: "...",
                expected: 123,
            },
            TestCase {
                input: "abc",
                expected: 345,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
