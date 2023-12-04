use regex::Regex;
use std::io::BufRead;

#[derive(Debug)]
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
    let res = process_data(ld);
    println!("{}", res);
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

fn process_data(ld: Vec<LineData>) -> u32 {
    let mut valid_parts: Vec<&Item> = Vec::new();
    for n in 0..ld.len() {
        if !ld[n].symbols.is_empty() {
            for symbol in &ld[n].symbols {
                let symbol_range = (&symbol.index) - 1..(&symbol.index) + 1;
                // check line above
                if n > 0 {
                    // can't check above the first line
                    fun_name(&ld, n - 1, &symbol_range, &mut valid_parts);
                }

                // check this line
                fun_name(&ld, n, &symbol_range, &mut valid_parts);

                // check line below
                fun_name(&ld, n + 1, &symbol_range, &mut valid_parts);
            }
        }
    }
    let x = valid_parts
        .iter()
        .fold(0, |acc, e| acc + e.value.parse::<u32>().unwrap());

    x
}

fn fun_name<'a>(
    ld: &'a [LineData],
    n: usize,
    symbol_range: &std::ops::Range<usize>,
    valid_parts: &mut Vec<&'a Item>,
) {
    for number in &*ld[n].numbers {
        let range = number.index..(number.index + number.value.len() - 1);
        if ranges_overlap(range, symbol_range.clone()) {
            println!("match! {:?}", number);
            valid_parts.push(number);
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
    let res = process_data(ld);
    println!("{}", res);
    assert_eq!(res, 4361);
}
