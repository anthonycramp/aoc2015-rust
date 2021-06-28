use regex::Regex;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("day14.txt");

fn main() {
    println!("Day 14 Part 1: {:?}", part1(INPUT));
    println!("Day 14 Part 2: {:?}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    Race::from(input).get_winning_distance(2503)
}

fn part2(input: &str) -> i32 {
    0
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Reindeer {
    name: String,
    speed: u32,
    speed_duration: u32,
    rest_duration: u32,
}

impl From<&str> for Reindeer {
    fn from(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(.*) can fly (.*) km/s for (.*) seconds, but then must rest for (.*) seconds."
            )
            .unwrap();
        }

        let fields = RE.captures(input).unwrap();

        Self {
            name: String::from(fields.get(1).unwrap().as_str()),
            speed: fields.get(2).unwrap().as_str().parse().unwrap(),
            speed_duration: fields.get(3).unwrap().as_str().parse().unwrap(),
            rest_duration: fields.get(4).unwrap().as_str().parse().unwrap(),
        }
    }
}

impl Reindeer {
    fn calculate_distance_travelled(&self, t: u32) -> u32 {
        let cycle_duration = self.speed_duration + self.rest_duration;
        let num_cycles = t / cycle_duration;
        let partial_end_cycle_duration = t % cycle_duration;
        let time_at_speed = num_cycles * self.speed_duration
            + std::cmp::min(self.speed_duration, partial_end_cycle_duration);
        self.speed * time_at_speed
    }
}

struct Race {
    reindeer: Vec<Reindeer>,
}

impl From<&str> for Race {
    fn from(input: &str) -> Self {
        let mut reindeer = vec![];
        for line in input.lines() {
            reindeer.push(Reindeer::from(line));
        }

        Self { reindeer }
    }
}

impl Race {
    fn get_winning_distance(&self, t: u32) -> u32 {
        self.reindeer
            .iter()
            .map(|r| r.calculate_distance_travelled(t))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_reindeer_parse() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let comet = Reindeer::from(input);

        assert_eq!(
            comet,
            Reindeer {
                name: String::from("Comet"),
                speed: 14,
                speed_duration: 10,
                rest_duration: 127
            }
        );

        let input = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let dancer = Reindeer::from(input);

        assert_eq!(
            dancer,
            Reindeer {
                name: String::from("Dancer"),
                speed: 16,
                speed_duration: 11,
                rest_duration: 162
            }
        );
    }

    #[test]
    fn test_distance_travelled() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let comet = Reindeer::from(input);

        assert_eq!(comet.calculate_distance_travelled(1000), 1120);

        let input = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let dancer = Reindeer::from(input);

        assert_eq!(dancer.calculate_distance_travelled(1000), 1056);
    }

    #[test]
    fn test_get_winning_distance() {
        let input = r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let race = Race::from(input);
        assert_eq!(race.get_winning_distance(1000), 1120);
    }
}
