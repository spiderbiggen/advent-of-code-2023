use std::collections::BTreeMap;

const INPUT: &str = include_str!("../puzzle_input");

fn parse_input(input: &str) -> (&str, BTreeMap<&str, (&str, &str)>) {
    let (directions, forks) = input.split_once("\n\n").unwrap();
    (directions, forks.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> (&str, (&str, &str)) {
    let mut split = line.split(" = ");
    let key = split.next().unwrap();
    let value = split.next().unwrap();
    let directions = value.trim_start_matches('(').trim_end_matches(')');
    (key, directions.split_once(", ").unwrap())
}

mod part_1 {
    fn solution(input: &str) -> u64 {
        let (directions, map) = super::parse_input(input);

        let directions = directions.chars().cycle().enumerate();
        let mut start = "AAA";
        let end = "ZZZ";
        for (index, direction) in directions {
            if direction == 'L' {
                start = map.get(start).unwrap().0;
            } else {
                start = map.get(start).unwrap().1;
            }
            if start == end {
                return index as u64 + 1;
            }
        }
        unreachable!()
    }

    const EXAMPLE_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_parse_example_input() {
        assert_eq!(solution(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_parse_second_example_input() {
        assert_eq!(solution(EXAMPLE_INPUT2), 6);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 8 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    fn solution(input: &str) -> u64 {
        let (directions, map) = super::parse_input(input);

        let directions = directions.chars().cycle();
        let mut nodes = map
            .keys()
            .copied()
            .filter(|key| key.ends_with('A'))
            .collect::<Vec<_>>();
        let mut step = 0u64;
        for direction in directions {
            step += 1;
            if direction == 'L' {
                nodes
                    .iter_mut()
                    .for_each(|key| *key = map.get(key).unwrap().0);
            } else {
                nodes
                    .iter_mut()
                    .for_each(|key| *key = map.get(key).unwrap().1);
            }
            if nodes.iter().all(|key| key.ends_with('Z')) {
                return step;
            }
        }
        unreachable!()
    }

    const EXAMPLE_INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_parse_example_input() {
        assert_eq!(solution(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 8 Part 2: [{}]", solution(super::INPUT));
    }
}
