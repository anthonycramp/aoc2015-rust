const INPUT: &str = include_str!("day12.txt");

fn main() {
    println!("Day 12 Part 1: {:?}", part1(INPUT));
    println!("Day 12 Part 2: {:?}", part2(INPUT));
}

fn is_json_punctuation(c: char) -> bool {
    r#"[]{},":"#.contains(c)
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    input
        .split(is_json_punctuation)
        .map(|s| if let Ok(v) = s.parse::<i32>() { v } else { 0 })
        .sum()
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
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
                input: "[1,2,3]",
                expected: 6,
            },
            TestCase {
                input: r#"{"a":2,"b":4}"#,
                expected: 6,
            },
            TestCase {
                input: r#"[[[3]]]"#,
                expected: 3,
            },
            TestCase {
                input: r#"{"a":{"b":4},"c":-1}"#,
                expected: 3,
            },
            TestCase {
                input: r#"{"a":[-1,1]}"#,
                expected: 0,
            },
            TestCase {
                input: r#"[-1,{"a":1}]"#,
                expected: 0,
            },
            TestCase {
                input: r#"[]"#,
                expected: 0,
            },
            TestCase {
                input: r#"{}"#,
                expected: 0,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    // #[test]
    // fn test_part2() {
    //     let test_cases = [
    //         TestCase {
    //             input: "...",
    //             expected: 123,
    //         },
    //         TestCase {
    //             input: "abc",
    //             expected: 345,
    //         },
    //     ];
    //     for TestCase { input, expected } in test_cases.iter() {
    //         assert_eq!(part2(*input), *expected);
    //     }
    // }
}
