use regex::Regex;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Item {
    index: usize,
    value: String,
}

struct LineData {
    numbers: Vec<Item>,
    symbols: Vec<Item>,
}

fn main() {
    let mut ld: Vec<LineData> = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let res = parse_line(line.unwrap().as_str());
        ld.push(res);
    }
    let (valid_parts, ratios) = process_data(ld);
    let sum = sum_valid_parts(valid_parts);
    println!("sum of parts = {} : gear ratios = {}", sum, ratios);
}

fn sum_valid_parts(parts: Vec<Item>) -> u32 {
    parts
        .iter()
        .fold(0, |acc, e| acc + e.value.parse::<u32>().unwrap())
}

fn parse_line(s: &str) -> LineData {
    let mut re = Regex::new(r"(\d+)+").unwrap();
    let numbers: Vec<Item> = collect_items(re, s);
    re = Regex::new(r"([^0-9.])").unwrap();
    let symbols = collect_items(re, s);
    LineData { numbers, symbols }
}

fn collect_items(re: Regex, s: &str) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    for mat in re.find_iter(s) {
        items.push(Item {
            index: mat.start(),
            value: String::from(&s[mat.start()..mat.end()]),
        });
    }
    items
}

fn process_data(ld: Vec<LineData>) -> (Vec<Item>, u32) {
    let mut valid_parts: Vec<Item> = Vec::new();
    let mut sum_ratios = 0;
    for n in 0..ld.len() {
        for symbol in &ld[n].symbols {
            let mut symbol_parts: Vec<Item> = Vec::new();
            let symbol_range = (&symbol.index) - 1..(&symbol.index) + 1;
            // check line above
            if n > 0 {
                // can't check above the first line
                find_valid_parts(&ld, n - 1, &symbol_range, &mut symbol_parts);
            }

            // check this line
            find_valid_parts(&ld, n, &symbol_range, &mut symbol_parts);

            // check line below
            find_valid_parts(&ld, n + 1, &symbol_range, &mut symbol_parts);

            if symbol_parts.len() == 2 {
                sum_ratios += symbol_parts[0].value.parse::<u32>().unwrap()
                    * symbol_parts[1].value.parse::<u32>().unwrap();
            }

            valid_parts.append(&mut symbol_parts);
        }
    }
    (valid_parts, sum_ratios)
}

fn find_valid_parts(
    ld: &[LineData],
    n: usize,
    symbol_range: &std::ops::Range<usize>,
    valid_parts: &mut Vec<Item>,
) {
    for number in &*ld[n].numbers {
        let range = number.index..(number.index + number.value.len() - 1);
        if ranges_overlap(range, symbol_range.clone()) {
            valid_parts.push(number.clone());
        }
    }
}

fn ranges_overlap<T: Ord>(range_one: std::ops::Range<T>, range_two: std::ops::Range<T>) -> bool {
    range_one.start <= range_two.end && range_two.start <= range_one.end
}

#[test]
fn test_schematic() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    let lines = input.split('\n');
    let mut ld: Vec<LineData> = Vec::new();
    for line in lines {
        let items = parse_line(line);
        ld.push(items);
    }
    let (valid_parts, ratios) = process_data(ld);
    let sum = sum_valid_parts(valid_parts);

    assert_eq!(sum, 4361);
    assert_eq!(ratios, 467835);
}
