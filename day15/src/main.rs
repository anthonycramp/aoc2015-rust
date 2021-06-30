use regex::Regex;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("day15.txt");

fn main() {
    println!("Day 15 Part 1: {:?}", part1(INPUT));
    println!("Day 15 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let ingredients = parse_ingredients(input);
    match ingredients.len() {
        2 => solve_2(&ingredients, part1_score_2),
        4 => solve_4(&ingredients, part1_score_4),
        _ => panic!("Can't handle {} ingredients", ingredients.len()),
    }
}

fn part1_score_2(ingredients: &[Ingredient], combinations: &[i32]) -> i32 {
    assert_eq!(ingredients.len(), combinations.len());

    let capacity = std::cmp::max(
        ingredients[0].capacity * combinations[0] + ingredients[1].capacity * combinations[1],
        0,
    );

    let durability = std::cmp::max(
        ingredients[0].durability * combinations[0] + ingredients[1].durability * combinations[1],
        0,
    );

    let flavour = std::cmp::max(
        ingredients[0].flavour * combinations[0] + ingredients[1].flavour * combinations[1],
        0,
    );

    let texture = std::cmp::max(
        ingredients[0].texture * combinations[0] + ingredients[1].texture * combinations[1],
        0,
    );

    capacity * durability * flavour * texture
}

fn part2_score_2(ingredients: &[Ingredient], combinations: &[i32]) -> i32 {
    let calories =
        ingredients[0].calories * combinations[0] + ingredients[1].calories * combinations[1];

    if calories == 500 {
        part1_score_2(&ingredients, &combinations)
    } else {
        0
    }
}

fn part1_score_4(ingredients: &[Ingredient], combinations: &[i32]) -> i32 {
    assert_eq!(ingredients.len(), combinations.len());

    let capacity = std::cmp::max(
        ingredients[0].capacity * combinations[0]
            + ingredients[1].capacity * combinations[1]
            + ingredients[2].capacity * combinations[2]
            + ingredients[3].capacity * combinations[3],
        0,
    );

    let durability = std::cmp::max(
        ingredients[0].durability * combinations[0]
            + ingredients[1].durability * combinations[1]
            + ingredients[2].durability * combinations[2]
            + ingredients[3].durability * combinations[3],
        0,
    );

    let flavour = std::cmp::max(
        ingredients[0].flavour * combinations[0]
            + ingredients[1].flavour * combinations[1]
            + ingredients[2].flavour * combinations[2]
            + ingredients[3].flavour * combinations[3],
        0,
    );

    let texture = std::cmp::max(
        ingredients[0].texture * combinations[0]
            + ingredients[1].texture * combinations[1]
            + ingredients[2].texture * combinations[2]
            + ingredients[3].texture * combinations[3],
        0,
    );

    capacity * durability * flavour * texture
}

fn part2_score_4(ingredients: &[Ingredient], combinations: &[i32]) -> i32 {
    let calories = ingredients[0].calories * combinations[0]
        + ingredients[1].calories * combinations[1]
        + ingredients[2].calories * combinations[2]
        + ingredients[3].calories * combinations[3];

    if calories == 500 {
        part1_score_4(&ingredients, &combinations)
    } else {
        0
    }
}

fn solve_2(ingredients: &[Ingredient], score_fn: fn(&[Ingredient], &[i32]) -> i32) -> i32 {
    assert_eq!(ingredients.len(), 2);
    let mut max_score = 0;
    for qty_1 in 0..=100 {
        let qty_2 = 100 - qty_1;

        let score = score_fn(ingredients, &vec![qty_1, qty_2]);

        max_score = std::cmp::max(score, max_score);
    }

    max_score
}

fn solve_4(ingredients: &[Ingredient], score_fn: fn(&[Ingredient], &[i32]) -> i32) -> i32 {
    assert_eq!(ingredients.len(), 4);
    let mut max_score = 0;

    for qty_1 in 0..=100 {
        for qty_2 in 0..=(100 - qty_1) {
            for qty_3 in 0..=(100 - qty_2 - qty_1) {
                let qty_4 = 100 - qty_3 - qty_2 - qty_1;

                let score = score_fn(&ingredients, &vec![qty_1, qty_2, qty_3, qty_4]);

                max_score = std::cmp::max(score, max_score);
            }
        }
    }
    max_score
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let ingredients = parse_ingredients(input);
    match ingredients.len() {
        2 => solve_2(&ingredients, part2_score_2),
        4 => solve_4(&ingredients, part2_score_4),
        _ => panic!("Can't handle {} ingredients", ingredients.len()),
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32,
}

impl From<&str> for Ingredient {
    fn from(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(.*): capacity (.*), durability (.*), flavor (.*), texture (.*), calories (.*)"
            )
            .unwrap();
        }
        let fields = RE.captures(input).unwrap();

        Self {
            name: String::from(fields.get(1).unwrap().as_str()),
            capacity: fields.get(2).unwrap().as_str().parse().unwrap(),
            durability: fields.get(3).unwrap().as_str().parse().unwrap(),
            flavour: fields.get(4).unwrap().as_str().parse().unwrap(),
            texture: fields.get(5).unwrap().as_str().parse().unwrap(),
            calories: fields.get(6).unwrap().as_str().parse().unwrap(),
        }
    }
}

fn parse_ingredients(input: &str) -> Vec<Ingredient> {
    input.lines().map(Ingredient::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let input = r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let first_line = input.lines().next().unwrap();

        assert_eq!(
            Ingredient::from(first_line),
            Ingredient {
                name: String::from("Butterscotch"),
                capacity: -1,
                durability: -2,
                flavour: 6,
                texture: 3,
                calories: 8,
            }
        );

        let ingredients = parse_ingredients(input);
        assert_eq!(ingredients.len(), 2);
        assert_eq!(ingredients[0].name, String::from("Butterscotch"));
        assert_eq!(ingredients[1].name, String::from("Cinnamon"));

        let max_score = part1(input);
        assert_eq!(max_score, 62842880);

        let max_score = part2(input);
        assert_eq!(max_score, 57600000);
    }
}
