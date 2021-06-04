const INPUT: &str = include_str!("inputs/day02.txt");

fn main() {
    println!("Day 02 Part 1: {}", part1(INPUT));
    println!("Day 02 Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| BoxDimensions::from(l).get_wrapping_paper_needed())
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| BoxDimensions::from(l).get_ribbon_needed())
        .sum()
}

struct BoxDimensions {
    width: u32,
    length: u32,
    height: u32,
}

impl BoxDimensions {
    fn get_wrapping_paper_needed(&self) -> u32 {
        let side_areas = [
            self.width * self.length,
            self.length * self.height,
            self.height * self.width,
        ];

        let minimum_side_area = side_areas.iter().min().unwrap();

        2 * side_areas[0] + 2 * side_areas[1] + 2 * side_areas[2] + minimum_side_area
    }

    fn get_ribbon_needed(&self) -> u32 {
        let side_perimetres = [
            2 * self.width + 2 * self.length,
            2 * self.length + 2 * self.height,
            2 * self.height + 2 * self.width,
        ];

        let minimum_side_perimetre = side_perimetres.iter().min().unwrap();

        minimum_side_perimetre + self.get_volume()
    }

    fn get_volume(&self) -> u32 {
        self.width * self.height * self.length
    }
}

impl From<&str> for BoxDimensions {
    fn from(input: &str) -> Self {
        let fields: Vec<_> = input.split('x').collect();
        Self {
            width: fields[0].parse().unwrap(),
            length: fields[1].parse().unwrap(),
            height: fields[2].parse().unwrap(),
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
                BoxDimensions::from(*input).get_wrapping_paper_needed(),
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
                BoxDimensions::from(*input).get_ribbon_needed(),
                *expected_output
            );
        }
    }
}
