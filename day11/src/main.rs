use std::collections::HashSet;

const INPUT: &str = "hepxcrrq";

fn main() {
    println!("Day 11 Part 1: {:?}", part1(INPUT));
    println!("Day 11 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> String {
    String::default()
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

trait Password {
    fn increment(&mut self);
    fn contains_three_letter_straight(&self) -> bool;
    fn contains_forbidden_characters(&self) -> bool;
    fn contains_different_pairs(&self) -> bool;
    fn is_valid_password_part1(&self) -> bool;
}

impl Password for String {
    fn increment(&mut self) {}

    fn contains_three_letter_straight(&self) -> bool {
        let mut bytes = self.bytes();
        let mut prev_byte = bytes.next().unwrap();
        let mut run_length = 0;
        while let Some(b) = bytes.next() {
            if (b - prev_byte) == 1
                || (prev_byte == b'h' && b == b'j')
                || (prev_byte == b'k' && b == b'm')
                || (prev_byte == b'n' && b == b'p')
            {
                run_length += 1;
            } else {
                run_length = 0;
            }

            if run_length == 2 {
                return true;
            }
            prev_byte = b;
        }

        false
    }

    fn contains_forbidden_characters(&self) -> bool {
        self.chars().any(|c| "ilo".contains(c))
    }

    fn contains_different_pairs(&self) -> bool {
        let mut pairs = HashSet::new();

        for w in self.as_bytes().windows(2) {
            if w[0] == w[1] {
                pairs.insert(w[0]);
            }
        }
        pairs.len() >= 2
    }

    fn is_valid_password_part1(&self) -> bool {
        self.contains_three_letter_straight()
            && !self.contains_forbidden_characters()
            && self.contains_different_pairs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_password_contains_three_letter_straight() {
        let test_cases = [
            TestCase {
                input: "hijklmmn",
                expected: true,
            },
            TestCase {
                input: "abbceffg",
                expected: false,
            },
            TestCase {
                input: "abbcegjk",
                expected: false,
            },
            TestCase {
                input: "abcdffaa",
                expected: true,
            },
            TestCase {
                input: "ghjaabcc",
                expected: true,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(
                String::from(*input).contains_three_letter_straight(),
                *expected
            );
        }
    }

    #[test]
    fn test_password_contains_forbidden_characters() {
        let test_cases = [
            TestCase {
                input: "hijklmmn",
                expected: true,
            },
            TestCase {
                input: "abbceffg",
                expected: false,
            },
            TestCase {
                input: "abbcegjk",
                expected: false,
            },
            TestCase {
                input: "abcdffaa",
                expected: false,
            },
            TestCase {
                input: "ghjaabcc",
                expected: false,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(
                String::from(*input).contains_forbidden_characters(),
                *expected
            );
        }
    }

    #[test]
    fn test_password_contains_different_pairs() {
        let test_cases = [
            TestCase {
                input: "hijklmmn",
                expected: false,
            },
            TestCase {
                input: "abbceffg",
                expected: true,
            },
            TestCase {
                input: "abbcegjk",
                expected: false,
            },
            TestCase {
                input: "abcdffaa",
                expected: true,
            },
            TestCase {
                input: "ghjaabcc",
                expected: true,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(String::from(*input).contains_different_pairs(), *expected);
        }
    }

    #[test]
    fn test_is_valid_password() {
        let test_cases = [
            TestCase {
                input: "hijklmmn",
                expected: false,
            },
            TestCase {
                input: "abbceffg",
                expected: false,
            },
            TestCase {
                input: "abbcegjk",
                expected: false,
            },
            TestCase {
                input: "abcdffaa",
                expected: true,
            },
            TestCase {
                input: "ghjaabcc",
                expected: true,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(String::from(*input).is_valid_password_part1(), *expected);
        }
    }

    #[test]
    fn test_part1() {
        let test_cases = [
            TestCase {
                input: "abcdefgh",
                expected: "abcdffaa",
            },
            TestCase {
                input: "ghijklmn",
                expected: "ghjaabcc",
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }
}
