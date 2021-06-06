const INPUT: &str = include_str!("inputs/day05.txt");

fn main() {
    println!("Day 05 Part 1: {}", part1(INPUT));
}

#[derive(Debug, PartialEq)]
enum Rating {
    Naughty,
    Nice,
}

fn contains_at_least_three_vowels(input: &str) -> bool {
    input.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

fn contains_at_least_one_double(input: &str) -> bool {
    let mut prev_char = ' ';

    for c in input.chars() {
        if prev_char == c {
            return true;
        }

        prev_char = c;
    }

    false
}

fn contains_forbidden_strings(input: &str) -> bool {
    input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
}

fn evaluate_string(input: &str) -> Rating {
    if contains_at_least_three_vowels(input)
        && contains_at_least_one_double(input)
        && !contains_forbidden_strings(input)
    {
        Rating::Nice
    } else {
        Rating::Naughty
    }
}

fn part1(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        if evaluate_string(line) == Rating::Nice {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        struct TestCase {
            input: &'static str,
            expected: Rating,
        }

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
            assert_eq!(evaluate_string(*input), *expected);
        }
    }
}
