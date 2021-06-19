const INPUT: &str = "1321131112";

fn main() {
    println!("Day 10 Part 1: {:?}", part1(INPUT));
    println!("Day 10 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> u64 {
    let mut ret = String::from(input);
    for _ in 1..=40 {
        ret = look_and_say(&ret);
    }
    ret.len() as u64
}

// replace return type as required by the problem
fn part2(input: &str) -> usize {
    let mut ret = String::from(input);

    for _ in 1..=50 {
        ret = look_and_say(&ret);
    }

    ret.len()
}

fn look_and_say(input: &str) -> String {
    let mut chars = input.chars();

    let mut last_seen = chars.next().unwrap();
    let mut count = 1;
    let mut ret = String::new();

    for c in chars {
        if c != last_seen {
            ret.push_str(&format!("{}{}", count, last_seen));
            last_seen = c;
            count = 1;
        } else {
            count += 1;
        }
    }

    ret.push_str(&format!("{}{}", count, last_seen));

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [
            TestCase {
                input: "1",
                expected: "11",
            },
            TestCase {
                input: "11",
                expected: "21",
            },
            TestCase {
                input: "21",
                expected: "1211",
            },
            TestCase {
                input: "1211",
                expected: "111221",
            },
            TestCase {
                input: "111221",
                expected: "312211",
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(look_and_say(*input), *expected);
        }
    }
}
