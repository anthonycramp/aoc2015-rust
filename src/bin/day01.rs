const INPUT: &str = include_str!("../../problem_inputs/day01.txt");

fn main() {
    println!("Day 01: {}", follow_directions(INPUT));
}

fn follow_directions(directions: &str) -> i32 {
    let floors_up = directions.chars().filter(|&c| c == '(').count() as i32;
    let floors_down = directions.chars().filter(|&c| c == ')').count() as i32;
    // this could be
    // let floors_down = directions.len() as i32 - floors_up;

    floors_up - floors_down
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(follow_directions(""), 0);
    }

    #[test]
    fn day01_problem_tests() {
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
}
