const INPUT: &str = "bgvyzdsv";

fn main() {
    println!("Day 04 Part 1: {}", part1(INPUT));
    println!("Day 04 Part 2: {}", part2(INPUT));
}

fn part1(key: &str) -> u32 {
    find_hash(key, "00000")
}

fn part2(key: &str) -> u32 {
    find_hash(key, "000000")
}

fn find_hash(key: &str, digest_prefix: &str) -> u32 {
    let mut number = 1;
    loop {
        let input = format!("{}{}", key, number);
        let digest = format!("{:x}", md5::compute(input));
        if digest.starts_with(digest_prefix) {
            break number;
        }

        number += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [
            TestCase {
                input: "abcdef",
                expected: 609043,
            },
            TestCase {
                input: "pqrstuv",
                expected: 1048970,
            },
        ];

        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }
}
