const INPUT: usize = 33100000;

fn main() {
    println!("Day 20 Part 1: {:?}", part1(INPUT));
    println!("Day 20 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: usize) -> usize {
    0
}

// replace return type as required by the problem
fn part2(input: usize) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [
            TestCase {
                input: 120,
                expected: 6,
            },
            TestCase {
                input: 130,
                expected: 9,
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
                input: 120,
                expected: 6,
            },
            TestCase {
                input: 130,
                expected: 9,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }
}
