const INPUT: &str = include_str!("day17.txt");

fn main() {
    println!("Day 17 Part 1: {:?}", part1(INPUT));
    println!("Day 17 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    0
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

fn parse(input: &str) -> Vec<u8> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "20
15
10
5
5";

        let container_sizes = parse(input);
        assert_eq!(container_sizes.len(), 5);
        assert_eq!(container_sizes[0], 20);
    }

    #[test]
    fn test_part2() {}
}
