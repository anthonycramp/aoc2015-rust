const INPUT: &str = include_str!("day18.txt");

fn main() {
    println!("Day 18 Part 1: {:?}", part1(INPUT));
    println!("Day 18 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    0
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[derive(Clone, Debug, PartialEq)]
enum LightState {
    On,
    Off,
}

#[derive(Clone, Debug)]
struct LightGrid {
    grid_size: usize,
    grid: Vec<Vec<LightState>>,
}

impl From<&str> for LightGrid {
    fn from(input: &str) -> Self {
        let grid_size = input.lines().count();
        let mut ret = Self {
            grid_size,
            grid: Vec::new(),
        };
        let mut lights_off_row = Vec::new();
        for _ in 0..(grid_size + 2) {
            lights_off_row.push(LightState::Off);
        }
        ret.grid.push(lights_off_row.clone());

        for line in input.lines() {
            let mut light_row = Vec::new();
            light_row.push(LightState::Off); // left border
            for c in line.chars() {
                match c {
                    '.' => light_row.push(LightState::Off),
                    '#' => light_row.push(LightState::On),
                    _ => panic!("Unknown light code: {}", c),
                }
            }
            light_row.push(LightState::Off); // right border
            ret.grid.push(light_row);
        }
        ret.grid.push(lights_off_row);

        ret
    }
}

impl LightGrid {
    fn count_lights_on(&self) -> usize {
        self.grid
            .iter()
            .map(|r| r.iter().filter(|&l| *l == LightState::On).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        let mut light_grid = LightGrid::from(input);
        assert_eq!(light_grid.count_lights_on(), 15);
        // light_grid.animate(4);
        // assert_eq!(light_grid.count_lights_on(), 4);
    }
}
