use itertools::Itertools;

const INPUT: &str = include_str!("day17.txt");

fn main() {
    println!("Day 17 Part 1: {:?}", part1(INPUT));
    println!("Day 17 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> u32 {
    let container_sizes = parse(input);
    count_combinations(150, &container_sizes)
}

// replace return type as required by the problem
fn part2(input: &str) -> u32 {
    let container_sizes = parse(input);
    count_minimum_combinations(150, &container_sizes)
}

fn collect_combinations(target: u32, bucket_sizes: &[u8]) -> Vec<Vec<u8>> {
    let mut combinations = Vec::new();

    for i in 1..=bucket_sizes.len() {
        let combos = bucket_sizes.into_iter().combinations(i);
        for combo in combos {
            if combo.iter().map(|&x| *x as u32).sum::<u32>() == target {
                combinations.push(combo.iter().copied().copied().collect::<Vec<_>>());
            }
        }
    }

    combinations
}

fn count_combinations(target: u32, bucket_sizes: &[u8]) -> u32 {
    collect_combinations(target, &bucket_sizes).len() as u32
}

fn count_minimum_combinations(target: u32, bucket_sizes: &[u8]) -> u32 {
    let combinations = collect_combinations(target, &bucket_sizes);
    let min_combinations = combinations.iter().map(|c| c.len()).min().unwrap();
    combinations
        .iter()
        .filter(|c| c.len() == min_combinations)
        .count() as u32
}

fn parse(input: &str) -> Vec<u8> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
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
    fn test_part1() {
        let input = "20
15
10
5
5";

        let combos = count_combinations(25, &parse(input));
        assert_eq!(combos, 4);
    }

    #[test]
    fn test_part2() {
        let input = "20
15
10
5
5";
        let combos = count_minimum_combinations(25, &parse(input));
        assert_eq!(combos, 3);
    }
}
