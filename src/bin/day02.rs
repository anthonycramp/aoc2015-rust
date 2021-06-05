use regex;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("inputs/day02.txt");

fn main() {
    println!("Day 02 Part 1: {}", part1(INPUT));
    println!("Day 02 Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| get_wrapping_paper_needed(&BoxDimensions::from(l)))
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| get_ribbon_needed(&BoxDimensions::from(l)))
        .sum()
}

struct BoxDimensions {
    width: u32,
    length: u32,
    height: u32,
}

impl BoxDimensions {
    fn get_volume(&self) -> u32 {
        self.width * self.height * self.length
    }
}

fn get_wrapping_paper_needed(box_dimensions: &BoxDimensions) -> u32 {
    let side_areas = [
        box_dimensions.width * box_dimensions.length,
        box_dimensions.length * box_dimensions.height,
        box_dimensions.height * box_dimensions.width,
    ];

    let minimum_side_area = side_areas.iter().min().unwrap();

    2 * side_areas[0] + 2 * side_areas[1] + 2 * side_areas[2] + minimum_side_area
}

fn get_ribbon_needed(box_dimensions: &BoxDimensions) -> u32 {
    let side_perimeters = [
        2 * box_dimensions.width + 2 * box_dimensions.length,
        2 * box_dimensions.length + 2 * box_dimensions.height,
        2 * box_dimensions.height + 2 * box_dimensions.width,
    ];

    let minimum_side_perimeter = side_perimeters.iter().min().unwrap();

    minimum_side_perimeter + box_dimensions.get_volume()
}

impl From<&str> for BoxDimensions {
    fn from(input: &str) -> Self {
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
        }

        let fields = RE.captures(input).unwrap();
        Self {
            width: fields.get(1).unwrap().as_str().parse().unwrap(),
            length: fields.get(2).unwrap().as_str().parse().unwrap(),
            height: fields.get(3).unwrap().as_str().parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_box_dimensions() {
        let box_dimensions = BoxDimensions::from("1x2x3");

        assert_eq!(box_dimensions.width, 1);
        assert_eq!(box_dimensions.length, 2);
        assert_eq!(box_dimensions.height, 3);
    }

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
            assert_eq!(
                get_wrapping_paper_needed(&BoxDimensions::from(*input)),
                *expected_output
            );
        }
    }

    #[test]
    fn test_part2() {
        struct TestCase {
            input: &'static str,
            expected_output: u32,
        }

        let test_cases = [
            TestCase {
                input: "2x3x4",
                expected_output: 34,
            },
            TestCase {
                input: "1x1x10",
                expected_output: 14,
            },
        ];

        for TestCase {
            input,
            expected_output,
        } in test_cases.iter()
        {
            assert_eq!(
                get_ribbon_needed(&BoxDimensions::from(*input)),
                *expected_output
            );
        }
    }
}
