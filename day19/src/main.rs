const INPUT: &str = include_str!("day19.txt");

fn main() {
    println!("Day 19 Part 1: {:?}", part1(INPUT));
    println!("Day 19 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    0
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

struct MoleculeMachine {}

impl From<&str> for MoleculeMachine {
    fn from(input: &str) -> Self {
        Self {}
    }
}

impl MoleculeMachine {
    fn count_molecules(&self) -> usize {
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replacement1() {
        let input = "H => HO
H => OH
O => HH

HOH";
        let machine = MoleculeMachine::from(input);
        assert_eq!(machine.count_molecules(), 4);
    }

    #[test]
    fn test_replacement2() {
        let input = "H => HO
H => OH
O => HH

HOHOHO";
        let machine = MoleculeMachine::from(input);
        assert_eq!(machine.count_molecules(), 7);
    }
}
