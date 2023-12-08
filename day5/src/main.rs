use std::cmp::Ordering;
use std::io::Read;
use std::ops::Range;

#[derive(Debug)]
struct Section<'a> {
    lines: Vec<&'a str>,
}

#[derive(Debug)]
struct SourceDestinationMap {
    source_range: Range<u64>,
    destination_range: Range<u64>,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let seed_line = lines.next().unwrap();

    let seed_numbers = get_seed_numbers(seed_line);

    let all_seeds = get_all_seeds(seed_numbers);
    // println!("{:?}", all_seeds);
    let lowest_location = parse_input(lines, all_seeds);
    println!("{}", lowest_location);
}

fn parse_input(lines: std::str::Lines<'_>, seed_numbers: Vec<u64>) -> u64 {
    let mut next_numbers = seed_numbers.clone();
    let sections = get_structured_map(lines);

    for section in sections {
        let mut maps: Vec<SourceDestinationMap> = Vec::new();
        let mut locations: Vec<u64> = Vec::new();
        for (i, line) in section.lines.iter().enumerate() {
            match i.cmp(&0) {
                Ordering::Equal => {
                    let seedparts: Vec<&str> = line.split(':').collect::<Vec<&str>>()[1]
                        .split_ascii_whitespace()
                        .collect();
                    if !seedparts.is_empty() {
                        next_numbers = seedparts
                            .iter()
                            .map(|p| p.parse::<u64>().unwrap())
                            .collect();
                    }
                }
                Ordering::Greater => {
                    let r_parts: Vec<&str> = line.split_ascii_whitespace().collect();
                    let start: u64 = r_parts[0].parse().unwrap();
                    let end: u64 = r_parts[1].parse().unwrap();
                    let size: u64 = r_parts[2].parse().unwrap();
                    let destination_range = start..start + size;
                    let source_range: std::ops::Range<u64> = end..end + size;
                    maps.push(SourceDestinationMap {
                        source_range,
                        destination_range,
                    });
                }
                Ordering::Less => {}
            }
        }

        if !maps.is_empty() {
            for seed in &next_numbers {
                let mut dest_value_op: Option<u64> = None;
                for map in &maps {
                    if map.source_range.contains(seed) {
                        let source_index = seed - map.source_range.start;
                        let dest_value = map.destination_range.start + source_index;
                        dest_value_op = Some(dest_value);
                        locations.push(dest_value);
                    }
                }
                if dest_value_op.is_none() {
                    locations.push(*seed);
                }
            }
            next_numbers = locations.clone();
        }
    }
    next_numbers.sort();
    next_numbers[0]
}

fn get_structured_map(lines: std::str::Lines<'_>) -> Vec<Section<'_>> {
    let mut sections: Vec<Section> = Vec::new();
    let mut section: Option<Section> = None;
    for line in lines {
        match section {
            None => section = Some(Section { lines: Vec::new() }),
            Some(_) => {}
        }

        if line.is_empty() {
            sections.push(section.unwrap());
            section = None;
        } else {
            section.as_mut().unwrap().lines.push(line);
        }
    }
    sections.push(section.unwrap());
    sections
}

fn get_seed_numbers(line: &str) -> Vec<u64> {
    let mut seed_numbers: Vec<u64> = Vec::new();
    let seedparts: Vec<&str> = line.split(':').collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .collect();
    if !seedparts.is_empty() {
        seed_numbers = seedparts
            .iter()
            .map(|p| p.parse::<u64>().unwrap())
            .collect();
    }
    seed_numbers
}

fn get_all_seeds(seed_numbers: Vec<u64>) -> Vec<u64> {
    let mut all_seeds: Vec<u64> = Vec::new();
    for (i, seed) in seed_numbers.iter().enumerate() {
        if (i + 1) % 2 == 0 {
            let r = seed_numbers[i - 1]..seed_numbers[i - 1] + *seed;

            println!("{:?}", r);
            all_seeds.append(&mut r.collect::<Vec<u64>>());
            // all_seeds.append(&mut items);
        }
    }
    all_seeds
}

#[cfg(test)]
mod tests {
    use crate::{get_all_seeds, get_seed_numbers, parse_input};

    const INPUT: &str = "seeds: 79 14 55 13

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
    #[test]
    fn part1() {
        let mut lines = INPUT.lines();
        let seed_numbers = get_seed_numbers(lines.next().unwrap());
        let lowest_location = parse_input(lines, seed_numbers);
        assert_eq!(lowest_location, 35);
    }

    #[test]
    fn part2() {
        let mut lines = INPUT.lines();
        let seed_line = lines.next().unwrap();
        let seed_numbers = get_seed_numbers(seed_line);

        let all_seeds = get_all_seeds(seed_numbers);

        let lowest_location = parse_input(lines, all_seeds);
        assert_eq!(lowest_location, 46);
    }
}
