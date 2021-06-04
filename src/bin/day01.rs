const INPUT: &str = include_str!("inputs/day01.txt");

fn main() {
    println!("Day 01 Part 1: {}", follow_directions(INPUT));
    println!(
        "Day 01 Part 2: {:?}",
        follow_directions_to_basement(INPUT).unwrap()
    );
}

fn follow_directions(directions: &str) -> i32 {
    let floors_up = directions.chars().filter(|&c| c == '(').count() as i32;
    let floors_down = directions.chars().filter(|&c| c == ')').count() as i32;
    // this could be
    // let floors_down = directions.len() as i32 - floors_up;

    floors_up - floors_down
}

fn follow_directions_to_basement(directions: &str) -> Option<usize> {
    let mut current_floor = 0;

    for (i, c) in directions.chars().enumerate() {
        match c {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => (),
        }

        if current_floor == -1 {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_part1() {
        assert_eq!(follow_directions(""), 0);
    }

    #[test]
    fn test_empty_part2() {
        assert!(follow_directions_to_basement("").is_none());
    }

    #[test]
    fn day01_problem_tests_part1() {
        struct Test {
            directions: &'static str,
            expected_floor: i32,
        }

        let test_cases = [
            Test {
                directions: "(())",
                expected_floor: 0,
            },
            Test {
                directions: "()()",
                expected_floor: 0,
            },
            Test {
                directions: "(((",
                expected_floor: 3,
            },
            Test {
                directions: "(()(()(",
                expected_floor: 3,
            },
            Test {
                directions: "))(((((",
                expected_floor: 3,
            },
            Test {
                directions: "())",
                expected_floor: -1,
            },
            Test {
                directions: "))(",
                expected_floor: -1,
            },
            Test {
                directions: ")))",
                expected_floor: -3,
            },
            Test {
                directions: ")())())",
                expected_floor: -3,
            },
        ];

        for Test {
            directions,
            expected_floor,
        } in test_cases.iter()
        {
            assert_eq!(follow_directions(directions), *expected_floor);
        }
    }

    #[test]
    fn day01_problem_tests_part2() {
        struct Test {
            directions: &'static str,
            basement_reached: Option<usize>,
        }

        let test_cases = [
            Test {
                directions: "(())",
                basement_reached: None,
            },
            Test {
                directions: "()()",
                basement_reached: None,
            },
            Test {
                directions: "(((",
                basement_reached: None,
            },
            Test {
                directions: "(()(()(",
                basement_reached: None,
            },
            Test {
                directions: "))(((((",
                basement_reached: Some(1),
            },
            Test {
                directions: "())",
                basement_reached: Some(3),
            },
            Test {
                directions: "))(",
                basement_reached: Some(1),
            },
            Test {
                directions: ")))",
                basement_reached: Some(1),
            },
            Test {
                directions: ")())())",
                basement_reached: Some(1),
            },
        ];

        for Test {
            directions,
            basement_reached,
        } in test_cases.iter()
        {
            assert_eq!(follow_directions_to_basement(directions), *basement_reached);
        }
    }
}
