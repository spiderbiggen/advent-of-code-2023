use std::collections::HashMap;

const INPUT: &str = include_str!("../puzzle_input");
const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Category {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl Iterator for Category {
    type Item = Category;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Category::SeedToSoil => Some(Self::SoilToFertilizer),
            Category::SoilToFertilizer => Some(Self::FertilizerToWater),
            Category::FertilizerToWater => Some(Self::WaterToLight),
            Category::WaterToLight => Some(Self::LightToTemperature),
            Category::LightToTemperature => Some(Self::TemperatureToHumidity),
            Category::TemperatureToHumidity => Some(Self::HumidityToLocation),
            Category::HumidityToLocation => None,
        }
    }
}

impl Category {
    fn from_str(s: &str) -> Self {
        match s {
            "seed-to-soil" => Category::SeedToSoil,
            "soil-to-fertilizer" => Category::SoilToFertilizer,
            "fertilizer-to-water" => Category::FertilizerToWater,
            "water-to-light" => Category::WaterToLight,
            "light-to-temperature" => Category::LightToTemperature,
            "temperature-to-humidity" => Category::TemperatureToHumidity,
            "humidity-to-location" => Category::HumidityToLocation,
            _ => panic!("invalid category: {s}"),
        }
    }
}

#[derive(Debug)]
struct CategoryRanges(Vec<CategoryRange>);

impl CategoryRanges {
    fn new(mut input: Vec<CategoryRange>) -> Self {
        input.sort_by(|a, b| a.source.cmp(&b.source).then(a.length.cmp(&b.length)));
        CategoryRanges(input)
    }

    fn resolve_number(&self, input: u64) -> u64 {
        match self.0.binary_search_by(|r| r.source.cmp(&input)) {
            Ok(index) => {
                let r = &self.0[index];
                r.target + input - r.source
            }
            Err(0) => input,
            Err(max_index) => {
                let range = &self.0[max_index - 1];
                if input < range.source + range.length {
                    range.target + input - range.source
                } else {
                    input
                }
            }
        }
    }
}

#[derive(Debug)]
struct CategoryRange {
    source: u64,
    target: u64,
    length: u64,
}

fn parse_input(input: &str) -> (Vec<u64>, HashMap<Category, CategoryRanges>) {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap());
    let map = parts.map(parse_map).collect();
    (seeds, map)
}

fn parse_seeds(input: &str) -> Vec<u64> {
    let (_, nums) = input.split_once(':').unwrap();
    nums.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_map(input: &str) -> (Category, CategoryRanges) {
    let (key, ranges) = input.split_once("map:\n").unwrap();
    let category = Category::from_str(key.trim());
    let ranges = ranges.lines().map(parse_range).collect();
    // eprintln!("{category:?}: {ranges:?}");
    (category, CategoryRanges::new(ranges))
}

fn parse_range(input: &str) -> CategoryRange {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    let [target, source, length] = parts[..] else {
        panic!("invalid range: {input}");
    };
    CategoryRange {
        source: source.parse().unwrap(),
        target: target.parse().unwrap(),
        length: length.parse().unwrap(),
    }
}

fn resolve_location(steps: &HashMap<Category, CategoryRanges>, seed: u64) -> u64 {
    let mut category = Category::SeedToSoil;
    let mut number = seed;
    // eprintln!("====");
    loop {
        let new_number = steps.get(&category).unwrap().resolve_number(number);
        // eprintln!("{category:?} {number}: {new_number}");
        number = new_number;

        match category.next() {
            Some(c) => category = c,
            None => return number,
        }
    }
}

mod part_1 {
    use crate::resolve_location;

    fn solution(input: &str) -> u64 {
        let (seeds, steps) = super::parse_input(input);
        seeds
            .iter()
            .copied()
            .map(|seeds| resolve_location(&steps, seeds))
            .min()
            .unwrap()
    }

    #[test]
    fn test_parse_example_input() {
        let input = super::EXAMPLE_INPUT;
        assert_eq!(solution(input), 35);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 5 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    use rayon::prelude::*;

    use crate::resolve_location;

    fn solution(input: &str) -> u64 {
        let (seeds, steps) = super::parse_input(input);
        let mut ranges = vec![];
        for i in (0..seeds.len()).step_by(2) {
            let range_start = seeds[i];
            let range_end = range_start + seeds[i + 1];
            let seed_range = range_start..range_end;
            ranges.push(seed_range);
        }
        ranges
            .into_par_iter()
            .flatten()
            .map(|r| resolve_location(&steps, r))
            .min()
            .unwrap()
    }

    #[test]
    fn test_parse_example_input() {
        let input = super::EXAMPLE_INPUT;
        assert_eq!(solution(input), 46);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 5 Part 2: [{}]", solution(super::INPUT));
    }
}
