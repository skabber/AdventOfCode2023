use std::{collections::HashMap, io::Read, str::Lines};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut lines = input.lines().clone();
    let (directions, map) = parse_lines(&mut lines);

    let count = loop_maps(&directions, &map, "AAA", "ZZZ");
    println!("Part 1: {}", count); //12361

    let starts: Vec<&str> = map.keys().filter(|e| e.ends_with(&"A")).copied().collect();
    let val = lcm_for_starts(starts, directions, map);
    println!("Part 2: {}", val); // 18215611419223
}

fn parse_lines<'a>(lines: &'a mut Lines<'a>) -> (Vec<char>, HashMap<&'a str, Vec<&'a str>>) {
    let directions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines {
        if !line.is_empty() {
            let line_parts: Vec<&str> = line.split('=').collect();
            if line_parts.len() == 2 {
                let key = line_parts[0].trim();
                let value = line_parts[1].trim();

                // Check if value starts with '(' and ends with ')'
                if value.starts_with('(') && value.ends_with(')') {
                    // Extract the substring without the parentheses
                    let inner_value = &value[1..value.len() - 1];

                    // Splitting by "," to get individual values
                    let value_parts: Vec<&str> = inner_value.split(',').collect();
                    let mut values = Vec::new();

                    for v in value_parts {
                        values.push(v.trim());
                    }

                    map.insert(key, values);
                }
            }
        }
    }
    (directions, map)
}

fn follow_map<'a>(
    directions: Vec<char>,
    map: HashMap<&'a str, Vec<&'a str>>,
    start_key: &str,
    _destination: &str,
) -> (&'a str, usize) {
    let (mut node_key, mut node_values) = map.get_key_value(start_key).unwrap();
    let mut step_count = 0;
    for d in directions {
        step_count += 1;
        match d {
            'L' => {
                (node_key, node_values) = map.get_key_value(node_values[0]).unwrap();
            }
            'R' => {
                (node_key, node_values) = map.get_key_value(node_values[1]).unwrap();
            }
            _ => {
                panic!("Unknown direction: {}", d);
            }
        }
    }
    (node_key, step_count)
}

fn loop_maps<'a>(
    directions: &[char],
    map: &HashMap<&'a str, Vec<&'a str>>,
    start: &str,
    destination: &str,
) -> usize {
    let mut node_key: &str = start;
    let mut count: usize = 0;
    loop {
        let (dest, c) = follow_map(directions.to_vec(), map.clone(), node_key, destination);
        count += c;

        if dest.ends_with(destination) {
            break;
        } else {
            node_key = dest;
        }
    }
    count
}

fn lcm_for_starts(
    starts: Vec<&str>,
    directions: Vec<char>,
    map: std::collections::HashMap<&str, Vec<&str>>,
) -> usize {
    let counts: Vec<usize> = starts
        .iter()
        .map(|n| loop_maps(&directions, &map, n, "Z"))
        .collect();

    lcm(&counts[..])
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use crate::{lcm_for_starts, loop_maps, parse_lines};

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1() {
        let mut lines = INPUT.lines();
        let (directions, map) = parse_lines(&mut lines);

        let count = loop_maps(&directions, &map, "AAA", "ZZZ");
        assert_eq!(count, 6_usize);
    }

    #[test]
    fn test_part2() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        let mut lines = input.lines();
        let (directions, map) = parse_lines(&mut lines);
        let starts: Vec<&str> = map.keys().filter(|e| e.ends_with(&"A")).copied().collect();
        let val = lcm_for_starts(starts, directions, map);
        assert_eq!(val, 6);
    }
}
