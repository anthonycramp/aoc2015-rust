const INPUT: &str = include_str!("inputs/day01.txt");

fn main() {
    println!("Day 01 Part 1: {}", follow_directions(INPUT));
    println!(
        "Day 01 Part 2: {:?}",
        follow_directions_to_basement(INPUT).unwrap()
    );
}

fn follow_directions(directions: &str) -> i32 {
    let floors_up = directions.chars().filter(|&c| c == '(').count() as i32;
    let floors_down = directions.chars().filter(|&c| c == ')').count() as i32;
    // this could be
    // let floors_down = directions.len() as i32 - floors_up;

    floors_up - floors_down
}

fn follow_directions_to_basement(directions: &str) -> Option<usize> {
    let mut current_floor = 0;

    for (i, c) in directions.chars().enumerate() {
        match c {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => (),
        }

        if current_floor == -1 {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2015::test_support::{run_tests, TestCase};

    #[test]
    fn test_empty_part1() {
        assert_eq!(follow_directions(""), 0);
    }

    #[test]
    fn test_empty_part2() {
        assert!(follow_directions_to_basement("").is_none());
    }

    #[test]
    fn day01_problem_tests_part1() {
        let test_cases = [
            TestCase {
                input: "(())",
                expected: 0,
            },
            TestCase {
                input: "()()",
                expected: 0,
            },
            TestCase {
                input: "(((",
                expected: 3,
            },
            TestCase {
                input: "(()(()(",
                expected: 3,
            },
            TestCase {
                input: "))(((((",
                expected: 3,
            },
            TestCase {
                input: "())",
                expected: -1,
            },
            TestCase {
                input: "))(",
                expected: -1,
            },
            TestCase {
                input: ")))",
                expected: -3,
            },
            TestCase {
                input: ")())())",
                expected: -3,
            },
        ];

        run_tests(follow_directions, &test_cases);
    }

    #[test]
    fn day01_problem_tests_part2() {
        run_tests(
            follow_directions_to_basement,
            &[
                TestCase {
                    input: "(())",
                    expected: None,
                },
                TestCase {
                    input: "()()",
                    expected: None,
                },
                TestCase {
                    input: "(((",
                    expected: None,
                },
                TestCase {
                    input: "(()(()(",
                    expected: None,
                },
                TestCase {
                    input: "))(((((",
                    expected: Some(1),
                },
                TestCase {
                    input: "())",
                    expected: Some(3),
                },
                TestCase {
                    input: "))(",
                    expected: Some(1),
                },
                TestCase {
                    input: ")))",
                    expected: Some(1),
                },
                TestCase {
                    input: ")())())",
                    expected: Some(1),
                },
            ],
        );
    }
}
