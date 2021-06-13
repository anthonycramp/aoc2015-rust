use std::collections::HashMap;

const INPUT: &str = include_str!("day05.txt");

fn main() {
    println!("Day 05 Part 1: {}", part1(INPUT));
    println!("Day 05 Part 2: {}", part2(INPUT));
}

#[derive(Debug, PartialEq)]
enum Rating {
    Naughty,
    Nice,
}

impl Rating {
    fn nice_if_true(v: bool) -> Self {
        if v {
            Rating::Nice
        } else {
            Rating::Naughty
        }
    }
}

impl From<bool> for Rating {
    fn from(v: bool) -> Self {
        if v {
            Rating::Nice
        } else {
            Rating::Naughty
        }
    }
}

fn contains_at_least_three_vowels(input: &str) -> bool {
    input.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

fn contains_at_least_one_double(input: &str) -> bool {
    input.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn contains_forbidden_strings(input: &str) -> bool {
    static FORBIDDEN_STRINGS: &[&str] = &["ab", "cd", "pq", "xy"];
    FORBIDDEN_STRINGS.iter().any(|f| input.contains(f))
}

fn evaluate_string_part1(input: &str) -> Rating {
    Rating::nice_if_true(
        contains_at_least_three_vowels(input)
            && contains_at_least_one_double(input)
            && !contains_forbidden_strings(input),
    )
}

fn count_nices(input: &str, evaluation_fn: fn(&str) -> Rating) -> u32 {
    input
        .lines()
        .map(|l| match evaluation_fn(l) {
            Rating::Nice => 1,
            _ => 0,
        })
        .sum()
}

fn part1(input: &str) -> u32 {
    count_nices(input, evaluate_string_part1)
}

fn contains_pair_of_two_letters(input: &str) -> bool {
    let mut pair_indices: HashMap<&str, Vec<usize>> = HashMap::new();

    for (i, w) in input.as_bytes().windows(2).enumerate() {
        let pair = std::str::from_utf8(w).unwrap();
        if let Some(indices) = pair_indices.get_mut(pair) {
            indices.push(i);
        } else {
            pair_indices.insert(pair, vec![i]);
        }
    }

    // I have a map with keys being pairs of characters and values being
    // the indexes of the pair in the original string. If a pair exists
    // more than once, it will have more than one index. For there to be
    // distinct pairs, there needs to be either three or more indexes, or
    // two indexes need to be separated by more than one position. These take
    // care of special cases such as "aaa", which will generate two indexes
    // [0,1], which fails the more than one position requirement. String
    // "aaaa" will result in three indexes [0,1,2], which is valid. String
    // "aabaa" will generate two indexes for "aa", [0,3], which is fine.

    for v in pair_indices.values() {
        let more_than_two_pairs = v.len() >= 3;
        let two_pairs_that_dont_overlap = v.len() == 2 && v[1] - v[0] > 1;

        if two_pairs_that_dont_overlap || more_than_two_pairs {
            return true;
        }
    }

    false
}

fn contains_repeat_separated_by_one(input: &str) -> bool {
    input.as_bytes().windows(3).any(|w| w[0] == w[2])
}

fn evaluate_string_part2(input: &str) -> Rating {
    Rating::from(contains_pair_of_two_letters(input) && contains_repeat_separated_by_one(input))
}

fn part2(input: &str) -> u32 {
    count_nices(input, evaluate_string_part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [
            TestCase {
                input: "ugknbfddgicrmopn",
                expected: Rating::Nice,
            },
            TestCase {
                input: "aaa",
                expected: Rating::Nice,
            },
            TestCase {
                input: "jchzalrnumimnmhp",
                expected: Rating::Naughty,
            },
            TestCase {
                input: "haegwjzuvuyypxyu",
                expected: Rating::Naughty,
            },
            TestCase {
                input: "dvszwmarrgswjxmb",
                expected: Rating::Naughty,
            },
        ];

        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(evaluate_string_part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [
            TestCase {
                input: "qjhvhtzxzqqjkmpb",
                expected: Rating::Nice,
            },
            TestCase {
                input: "xxyxx",
                expected: Rating::Nice,
            },
            TestCase {
                input: "uurcxstgmygtbstg",
                expected: Rating::Naughty,
            },
            TestCase {
                input: "ieodomkazucvgmuy",
                expected: Rating::Naughty,
            },
            TestCase {
                input: "dvszwmarrgswjxmb",
                expected: Rating::Naughty,
            },
        ];

        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(evaluate_string_part2(*input), *expected);
        }
    }
}
