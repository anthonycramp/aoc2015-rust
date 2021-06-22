use std::collections::HashSet;

const INPUT: &str = "hepxcrrq";

fn main() {
    println!("Day 11 Part 1: {:?}", part1(INPUT));
    println!("Day 11 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> String {
    let mut new_password = String::from(input).increment();

    while !new_password.is_valid_password_part1() {
        new_password = new_password.increment();
    }

    new_password
}

// replace return type as required by the problem
fn part2(input: &str) -> String {
    let new_password = part1(input);
    // and then get the next one
    part1(&new_password)
}

trait Password {
    fn increment(self) -> Self;
    fn contains_three_letter_straight(&self) -> bool;
    fn contains_forbidden_characters(&self) -> bool;
    fn contains_different_pairs(&self) -> bool;
    fn is_valid_password_part1(&self) -> bool;
}

impl Password for String {
    fn increment(self) -> Self {
        let mut ret = String::new();
        let mut bubble_up = true;

        for b in self.bytes().rev() {
            if bubble_up {
                if b == b'z' {
                    ret = String::from("a") + &ret;
                    bubble_up = true;
                } else {
                    let mut new_b = b + 1;
                    if "ilo".contains(new_b as char) {
                        // skip illegal characters
                        new_b += 1;
                    }
                    ret = String::from(new_b as char) + &ret;
                    bubble_up = false;
                }
            } else {
                ret = String::from(b as char) + &ret;
            }
        }
        ret
    }

    fn contains_three_letter_straight(&self) -> bool {
        for straights in self.as_bytes().windows(3) {
            if straights[2] == (straights[1] + 1) && straights[1] == (straights[0] + 1) {
                return true;
            }
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
    fn test_increment() {
        let test_cases = [
            TestCase {
                input: "aaaaaaaa",
                expected: "aaaaaaab",
            },
            TestCase {
                input: "aaaaaaaz",
                expected: "aaaaaaba",
            },
            TestCase {
                input: "zzzzzzzz",
                expected: "aaaaaaaa",
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(String::from(*input).increment(), *expected);
        }
    }

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
