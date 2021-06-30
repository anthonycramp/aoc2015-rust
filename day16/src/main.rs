use regex::Regex;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("day16.txt");

fn main() {
    println!("Day 16 Part 1: {:?}", part1(INPUT));
    println!("Day 16 Part 2: {:?}", part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> u32 {
    let mut aunt_sues = Vec::new();
    for line in input.lines() {
        aunt_sues.push(AuntSue::from(line));
    }

    let scan = AuntSue::new(3, 7, 2, 3, 0, 0, 5, 3, 2, 1);

    if let Some(aunt_sue_match) = aunt_sues.iter().find(|&s| *s == scan) {
        return aunt_sue_match.id;
    }

    0
}

// replace return type as required by the problem
fn part2(input: &str) -> u32 {
    let mut aunt_sues = Vec::new();
    for line in input.lines() {
        aunt_sues.push(AuntSue::from(line));
    }

    let scan = AuntSue::new(3, 7, 2, 3, 0, 0, 5, 3, 2, 1);

    if let Some(aunt_sue_match) = aunt_sues.iter().find(|&s| comp_part2(s, &scan)) {
        return aunt_sue_match.id;
    }

    0
}

#[derive(Debug)]
struct AuntSue {
    id: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl AuntSue {
    fn new(
        children: u32,
        cats: u32,
        samoyeds: u32,
        pomeranians: u32,
        akitas: u32,
        vizslas: u32,
        goldfish: u32,
        trees: u32,
        cars: u32,
        perfumes: u32,
    ) -> Self {
        Self {
            id: 0,
            children: Some(children),
            cats: Some(cats),
            samoyeds: Some(samoyeds),
            pomeranians: Some(pomeranians),
            akitas: Some(akitas),
            vizslas: Some(vizslas),
            goldfish: Some(goldfish),
            trees: Some(trees),
            cars: Some(cars),
            perfumes: Some(perfumes),
        }
    }
}
impl Default for AuntSue {
    fn default() -> Self {
        Self {
            id: 0,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }
}

impl From<&str> for AuntSue {
    fn from(input: &str) -> Self {
        lazy_static! {
            static ref RE_outer: Regex = Regex::new(r"Sue (\d+): (.*)").unwrap();
            static ref RE_compound: Regex = Regex::new(r"(.*): (.*)").unwrap();
        }

        let outer_fields = RE_outer.captures(input).unwrap();

        let mut aunt_sue = AuntSue::default();
        aunt_sue.id = outer_fields.get(1).unwrap().as_str().parse().unwrap();

        let compounds = outer_fields.get(2).unwrap().as_str();

        for compound in compounds.split(", ") {
            let compoound_fields = RE_compound.captures(compound).unwrap();
            let compound_name = compoound_fields.get(1).unwrap().as_str();
            let compound_qty: u32 = compoound_fields.get(2).unwrap().as_str().parse().unwrap();

            match compound_name {
                "children" => aunt_sue.children = Some(compound_qty),
                "cats" => aunt_sue.cats = Some(compound_qty),
                "samoyeds" => aunt_sue.samoyeds = Some(compound_qty),
                "pomeranians" => aunt_sue.pomeranians = Some(compound_qty),
                "akitas" => aunt_sue.akitas = Some(compound_qty),
                "vizslas" => aunt_sue.vizslas = Some(compound_qty),
                "goldfish" => aunt_sue.goldfish = Some(compound_qty),
                "trees" => aunt_sue.trees = Some(compound_qty),
                "cars" => aunt_sue.cars = Some(compound_qty),
                "perfumes" => aunt_sue.perfumes = Some(compound_qty),
                _ => panic!("Unknown compound: {} {}", compound_name, compound_qty),
            }
        }

        aunt_sue
    }
}

impl PartialEq for AuntSue {
    fn eq(&self, other: &Self) -> bool {
        let children_match = self.children.is_none()
            || other.children.is_none()
            || (self.children == other.children);
        let cats_match = self.cats.is_none() || other.cats.is_none() || (self.cats == other.cats);
        let samoyeds_match = self.samoyeds.is_none()
            || other.samoyeds.is_none()
            || (self.samoyeds == other.samoyeds);
        let pomeranians_match = self.pomeranians.is_none()
            || other.pomeranians.is_none()
            || (self.pomeranians == other.pomeranians);
        let akitas_match =
            self.akitas.is_none() || other.akitas.is_none() || (self.akitas == other.akitas);
        let vizslas_match =
            self.vizslas.is_none() || other.vizslas.is_none() || (self.vizslas == other.vizslas);
        let goldfish_match = self.goldfish.is_none()
            || other.goldfish.is_none()
            || (self.goldfish == other.goldfish);
        let trees_match =
            self.trees.is_none() || other.trees.is_none() || (self.trees == other.trees);
        let cars_match = self.cars.is_none() || other.cars.is_none() || (self.cars == other.cars);
        let perfumes_match = self.perfumes.is_none()
            || other.perfumes.is_none()
            || (self.perfumes == other.perfumes);

        children_match
            && cats_match
            && samoyeds_match
            && pomeranians_match
            && akitas_match
            && vizslas_match
            && goldfish_match
            && trees_match
            && cars_match
            && perfumes_match
    }
}

fn comp_part2(aunt_sue: &AuntSue, scan: &AuntSue) -> bool {
    let children_match = aunt_sue.children.is_none()
        || scan.children.is_none()
        || (aunt_sue.children == scan.children);
    let cats_match = aunt_sue.cats.is_none() || scan.cats.is_none() || (aunt_sue.cats > scan.cats);
    let samoyeds_match = aunt_sue.samoyeds.is_none()
        || scan.samoyeds.is_none()
        || (aunt_sue.samoyeds == scan.samoyeds);
    let pomeranians_match = aunt_sue.pomeranians.is_none()
        || scan.pomeranians.is_none()
        || (aunt_sue.pomeranians < scan.pomeranians);
    let akitas_match =
        aunt_sue.akitas.is_none() || scan.akitas.is_none() || (aunt_sue.akitas == scan.akitas);
    let vizslas_match =
        aunt_sue.vizslas.is_none() || scan.vizslas.is_none() || (aunt_sue.vizslas == scan.vizslas);
    let goldfish_match = aunt_sue.goldfish.is_none()
        || scan.goldfish.is_none()
        || (aunt_sue.goldfish < scan.goldfish);
    let trees_match =
        aunt_sue.trees.is_none() || scan.trees.is_none() || (aunt_sue.trees > scan.trees);
    let cars_match = aunt_sue.cars.is_none() || scan.cars.is_none() || (aunt_sue.cars == scan.cars);
    let perfumes_match = aunt_sue.perfumes.is_none()
        || scan.perfumes.is_none()
        || (aunt_sue.perfumes == scan.perfumes);

    children_match
        && cats_match
        && samoyeds_match
        && pomeranians_match
        && akitas_match
        && vizslas_match
        && goldfish_match
        && trees_match
        && cars_match
        && perfumes_match
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Sue 483: pomeranians: 3, vizslas: 6, goldfish: 5";
        let aunt_sue = AuntSue::from(input);
        assert_eq!(aunt_sue.id, 483);
        assert_eq!(aunt_sue.children, None);
        assert_eq!(aunt_sue.pomeranians, Some(3));
        assert_eq!(aunt_sue.vizslas, Some(6));
        assert_eq!(aunt_sue.goldfish, Some(5));
        assert_eq!(aunt_sue.cars, None);
    }

    #[test]
    fn test_compare() {
        let input = "Sue 483: pomeranians: 3, vizslas: 6, goldfish: 5";
        let aunt_sue = AuntSue::from(input);

        let scan = AuntSue::new(3, 7, 2, 3, 0, 6, 5, 3, 2, 1);

        assert_eq!(scan, aunt_sue);
    }
}
