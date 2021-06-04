const INPUT: &str = include_str!("inputs/day02.txt");

fn main() {}

fn part1(input: &str) -> u32 {
    0
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
                input: "2x3x4",
                expected_output: 58,
            },
            TestCase {
                input: "1x1x10",
                expected_output: 43,
            },
        ];

        for TestCase {
            input,
            expected_output,
        } in test_cases.iter()
        {
            assert_eq!(part1(input), *expected_output);
        }
    }
}
