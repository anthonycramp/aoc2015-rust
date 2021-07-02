const INPUT: &str = include_str!("day18.txt");

fn main() {
    println!("Day 18 Part 1: {:?}", part1(INPUT));
    println!("Day 18 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> usize {
    let mut light_grid = LightGrid::from(input);
    (0..100).for_each(|_| light_grid.animate());
    light_grid.count_lights_on()
}

// replace return type as required by the problem
fn part2(input: &str) -> usize {
    let mut light_grid = LightGrid::from2(input);
    (0..100).for_each(|_| light_grid.animate2());
    light_grid.count_lights_on()
}

#[derive(Clone, Debug, PartialEq)]
enum LightState {
    On,
    Off,
}

#[derive(Clone)]
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

impl std::fmt::Display for LightGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for rows in self.grid.iter() {
            for light in rows.iter() {
                let char_to_write = match light {
                    LightState::On => '#',
                    LightState::Off => '.',
                };
                output.push(char_to_write);
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

impl LightGrid {
    fn from2(input: &str) -> Self {
        let mut light_grid = Self::from(input);

        // set corners to on
        light_grid.grid[1][1] = LightState::On;
        light_grid.grid[1][light_grid.grid_size] = LightState::On;
        light_grid.grid[light_grid.grid_size][1] = LightState::On;
        light_grid.grid[light_grid.grid_size][light_grid.grid_size] = LightState::On;

        light_grid
    }

    fn count_lights_on(&self) -> usize {
        self.grid
            .iter()
            .map(|r| r.iter().filter(|&l| *l == LightState::On).count())
            .sum()
    }

    fn count_neighbour_lights_on(&self, row: usize, col: usize) -> usize {
        let neighbours = [
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ];
        neighbours
            .iter()
            .filter(|(r, c)| self.grid[*r][*c] == LightState::On)
            .count()
    }

    fn animate(&mut self) {
        let grid_clone = self.clone();

        for row in 1..=self.grid_size {
            for col in 1..=self.grid_size {
                let neighbour_lights_on = grid_clone.count_neighbour_lights_on(row, col);
                let new_light_state = match grid_clone.grid[row][col] {
                    LightState::On => match neighbour_lights_on {
                        2 | 3 => LightState::On,
                        _ => LightState::Off,
                    },
                    LightState::Off => match neighbour_lights_on {
                        3 => LightState::On,
                        _ => LightState::Off,
                    },
                };
                self.grid[row][col] = new_light_state;
            }
        }
    }

    fn animate2(&mut self) {
        self.animate();
        // reset corners to on
        self.grid[1][1] = LightState::On;
        self.grid[1][self.grid_size] = LightState::On;
        self.grid[self.grid_size][1] = LightState::On;
        self.grid[self.grid_size][self.grid_size] = LightState::On;
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
        light_grid.animate();
        assert_eq!(light_grid.count_lights_on(), 11);
        light_grid.animate();
        assert_eq!(light_grid.count_lights_on(), 8);
        light_grid.animate();
        assert_eq!(light_grid.count_lights_on(), 4);
        light_grid.animate();
        assert_eq!(light_grid.count_lights_on(), 4);
    }

    #[test]
    fn test_animation2() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        let mut light_grid = LightGrid::from2(input);
        assert_eq!(light_grid.count_lights_on(), 17);
        light_grid.animate2();
        assert_eq!(light_grid.count_lights_on(), 18);
        light_grid.animate2();
        assert_eq!(light_grid.count_lights_on(), 18);
        light_grid.animate2();
        assert_eq!(light_grid.count_lights_on(), 18);
        light_grid.animate2();
        assert_eq!(light_grid.count_lights_on(), 14);
        light_grid.animate2();
        assert_eq!(light_grid.count_lights_on(), 17);
    }
}
