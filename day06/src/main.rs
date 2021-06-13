use regex::Regex;

const INPUT: &str = include_str!("day06.txt");

fn main() {
    println!("Day 06 Part 1: {}", part1(INPUT));
}

fn part1(input: &str) -> u32 {
    let mut lights = [0u8; 1_000_000];

    for line in input.lines() {
        let action = Action::from(line);
        action.work(&mut lights);
    }
    lights.iter().map(|&v| v as u32).sum()
}

#[macro_use]
extern crate lazy_static;

struct Coordinate {
    x: usize,
    y: usize,
}

struct Rectangle {
    lower_left: Coordinate,
    upper_right: Coordinate,
}

enum Action {
    On(Rectangle),
    Off(Rectangle),
    Toggle(Rectangle),
}

impl From<&str> for Action {
    fn from(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(.+) (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})").unwrap();
        }

        let fields = RE.captures(input).unwrap();
        let rect = Rectangle {
            lower_left: Coordinate {
                x: fields.get(2).unwrap().as_str().parse().unwrap(),
                y: fields.get(3).unwrap().as_str().parse().unwrap(),
            },
            upper_right: Coordinate {
                x: fields.get(4).unwrap().as_str().parse().unwrap(),
                y: fields.get(5).unwrap().as_str().parse().unwrap(),
            },
        };

        let command = fields.get(1).unwrap().as_str();
        match command {
            "turn on" => Action::On(rect),
            "turn off" => Action::Off(rect),
            "toggle" => Action::Toggle(rect),
            _ => panic!("Unknown command: {}", command),
        }
    }
}

fn compute_light_index(x: usize, y: usize) -> usize {
    x + (y * 1000)
}

impl Action {
    fn work(&self, lights: &mut [u8]) {
        match self {
            Action::On(rect) => {
                for x in rect.lower_left.x..=rect.upper_right.x {
                    for y in rect.lower_left.y..=rect.upper_right.y {
                        lights[compute_light_index(x, y)] = 1;
                    }
                }
            }
            Action::Off(rect) => {
                for x in rect.lower_left.x..=rect.upper_right.x {
                    for y in rect.lower_left.y..=rect.upper_right.y {
                        lights[compute_light_index(x, y)] = 0;
                    }
                }
            }
            Action::Toggle(rect) => {
                for x in rect.lower_left.x..=rect.upper_right.x {
                    for y in rect.lower_left.y..=rect.upper_right.y {
                        lights[compute_light_index(x, y)] = 1 - lights[compute_light_index(x, y)];
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_on() {
        let input = "turn on 0,0 through 999,999";
        let action = Action::from(input);
        match action {
            Action::On(rect) => {
                assert_eq!(rect.lower_left.x, 0);
                assert_eq!(rect.lower_left.y, 0);
                assert_eq!(rect.upper_right.x, 999);
                assert_eq!(rect.upper_right.y, 999);
            }
            _ => panic!("Did not decode an On action"),
        }
    }

    #[test]
    fn test_parse_off() {
        let input = "turn off 499,499 through 500,500";
        let action = Action::from(input);
        match action {
            Action::Off(rect) => {
                assert_eq!(rect.lower_left.x, 499);
                assert_eq!(rect.lower_left.y, 499);
                assert_eq!(rect.upper_right.x, 500);
                assert_eq!(rect.upper_right.y, 500);
            }
            _ => panic!("Did not decode an Off action"),
        }
    }

    #[test]
    fn test_parse_toggle() {
        let input = "toggle 0,0 through 999,0";
        let action = Action::from(input);
        match action {
            Action::Toggle(rect) => {
                assert_eq!(rect.lower_left.x, 0);
                assert_eq!(rect.lower_left.y, 0);
                assert_eq!(rect.upper_right.x, 999);
                assert_eq!(rect.upper_right.y, 0);
            }
            _ => panic!("Did not decode a Toggle action"),
        }
    }
}
