use std::{collections::HashMap, io::Read, str::Lines};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut lines = input.lines().clone();
    let (directions, map) = parse_lines(&mut lines);
    let count = loop_map(directions, map);
    println!("Part 1: {}", count); //12361
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
) -> (&'a str, u64) {
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
        if node_key == &"ZZZ" {
            break;
        }
    }
    (node_key, step_count)
}

fn loop_map<'a>(directions: Vec<char>, map: HashMap<&'a str, Vec<&'a str>>) -> u64 {
    let mut node_key = "AAA";
    let mut count = 0;
    let mut loop_count = 0;
    loop {
        loop_count += 1;
        let (dest, c) = follow_map(directions.clone(), map.clone(), node_key);
        count += c;
        if dest == "ZZZ" {
            break;
        } else {
            node_key = dest;
        }
    }
    println!("Loop count: {}", loop_count);
    count
}

#[cfg(test)]
mod tests {
    use crate::{loop_map, parse_lines};

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1() {
        let mut lines = INPUT.lines();
        let (directions, map) = parse_lines(&mut lines);
        let count = loop_map(directions, map);
        assert_eq!(count, 6_u64);
    }
}
