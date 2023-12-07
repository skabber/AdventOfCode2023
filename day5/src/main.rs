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
    let val = parse_input(input.as_str());
    println!("{}", val);
}

fn parse_input(input: &str) -> u64 {
    let sections = get_structured_map(input);
    let mut seed_numbers: Vec<u64> = Vec::new();

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
                        seed_numbers = seedparts
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
                    println!("about to {}", end);
                    let source_range: std::ops::Range<u64> = end..end + size;
                    maps.push(SourceDestinationMap {
                        source_range,
                        destination_range,
                    });
                }
                Ordering::Less => {}
            }
        }
        println!("{:?}", maps);

        for seed in &seed_numbers {
            let mut dest_value_op: Option<u64> = None;
            for map in &maps {
                if map.source_range.contains(seed) {
                    let source_index = seed - map.source_range.start;
                    let dest_value = map.destination_range.start + source_index;
                    dest_value_op = Some(dest_value);
                    locations.push(dest_value);
                    println!("{:?}", locations);
                }
            }
            if dest_value_op.is_none() {
                locations.push(*seed);
            }
        }
        seed_numbers = locations.clone();
    }
    seed_numbers.sort();
    seed_numbers[0]
}

fn get_structured_map(input: &str) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    let mut section: Option<Section> = None;
    for line in input.split('\n') {
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

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn test1() {
        let input = "seeds: 79 14 55 13

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
        let lowest_location = parse_input(input);
        assert_eq!(lowest_location, 35);
    }
}
