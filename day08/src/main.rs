const INPUT: &str = include_str!("day08.txt");

fn main() {
    println!("Day 08 Part 1: {:?}", part1(INPUT));
    println!("Day 08 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let code_chars_count: i32 = input.lines().map(|l| count_code_chars(l)).sum();
    let string_chars_count: i32 = input.lines().map(|l| count_string_chars(l)).sum();

    code_chars_count - string_chars_count
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let code_chars_count: i32 = input.lines().map(|l| count_code_chars(l)).sum();
    let encoded_code_chars_count: i32 = input
        .lines()
        .map(|l| count_code_chars(&encode_string(l)))
        .sum();
    encoded_code_chars_count - code_chars_count
}

fn count_code_chars(input: &str) -> i32 {
    input.bytes().len() as i32
}

fn count_string_chars(input: &str) -> i32 {
    let s = String::from(input);
    let len = s.len();
    let s = &s[1..(len - 1)];
    let mut count = 0;
    let mut in_escape = false;
    let mut chars_iter = s.chars();

    loop {
        let c = if let Some(c) = chars_iter.next() {
            c
        } else {
            break;
        };

        if !in_escape {
            count += 1;
            match c {
                '\\' => in_escape = true,
                _ => (),
            }
        } else {
            match c {
                'x' => {
                    chars_iter.next();
                    chars_iter.next();
                }
                _ => (),
            }
            in_escape = false;
        }
    }
    count as i32
}

fn encode_string(input: &str) -> String {
    let mut out = String::from("\"");
    for c in input.chars() {
        let encoding = match c {
            '"' => String::from(r#"\""#),
            '\\' => String::from(r#"\\"#),
            _ => String::from(c),
        };
        out.push_str(&encoding);
    }
    out.push_str("\"");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_count_code_chars() {
        let test_cases = [
            TestCase {
                input: r#""""#,
                expected: 2,
            },
            TestCase {
                input: r#""abc""#,
                expected: 5,
            },
            TestCase {
                input: r#""aaa\"aaa""#,
                expected: 10,
            },
            TestCase {
                input: r#""\x27""#,
                expected: 6,
            },
        ];

        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(count_code_chars(*input), *expected);
        }
    }

    #[test]
    fn test_count_string_chars() {
        let test_cases = [
            TestCase {
                input: r#""""#,
                expected: 0,
            },
            TestCase {
                input: r#""abc""#,
                expected: 3,
            },
            TestCase {
                input: r#""aaa\"aaa""#,
                expected: 7,
            },
            TestCase {
                input: r#""\x27""#,
                expected: 1,
            },
        ];

        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(count_string_chars(*input), *expected);
        }
    }

    #[test]
    fn test_part1() {
        let input = r##"""
"abc"
"aaa\"aaa"
"\x27""##;
        assert_eq!(part1(input), 12);
    }

    #[test]
    fn test_encode_string() {
        let test_cases = [
            TestCase {
                input: r#""""#,
                expected: r#""\"\"""#,
            },
            TestCase {
                input: r#""abc""#,
                expected: r#""\"abc\"""#,
            },
            TestCase {
                input: r#""aaa\"aaa""#,
                expected: r#""\"aaa\\\"aaa\"""#,
            },
            TestCase {
                input: r#""\x27""#,
                expected: r#""\"\\x27\"""#,
            },
        ];

        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(encode_string(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let input = r##"""
"abc"
"aaa\"aaa"
"\x27""##;
        assert_eq!(part2(input), 19);
    }
}
