use regex::Regex;

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

fn main() {
    Action::from("turn on 0,0 through 999,999");
}
