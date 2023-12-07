struct Map<'a> {
    section: Section<'a>,
}

#[derive(Debug)]
struct Section<'a> {
    lines: Vec<&'a str>,
}

fn main() {
    println!("Hello, world!");
}

fn parse_input(input: &str) -> u32 {
    let mut lowest_location = 0;
    let sections = get_structured_map(input);
    for section in sections {
        for (i, line) in section.lines.iter().enumerate() {
            if i == 0 {
                let seedparts = line.split(':').collect::<Vec<&str>>()[1];
            }

            // if line.split(':').collect() {}
            if i > 0 {
                let r_parts: Vec<&str> = line.split_ascii_whitespace().collect();
                let start: u32 = r_parts[0].parse().unwrap();
                let end: u32 = r_parts[1].parse().unwrap();
                let size: u32 = r_parts[2].parse().unwrap();
                let range_1 = start..size;
                let range_2 = end..size;
            }
        }
    }
    lowest_location
}

fn get_structured_map(input: &str) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    let mut section: Option<Section> = None;
    for line in input.split('\n') {
        match section {
            None => { section = Some(Section { lines: Vec::new() }) }
            Some(_) => {}
        }

        if line == "" {
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


