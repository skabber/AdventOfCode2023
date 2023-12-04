use std::io::BufRead;

fn main() {
    let mut val = 0;
    for line in std::io::stdin().lock().lines() {
        val += parse_card(line.unwrap().as_str());
    }
    println!("{}", val);
}

fn parse_card(line: &str) -> usize {
    let mut line_parts = line.split(':');
    let _card_name = line_parts.next().unwrap();
    let card_data = line_parts.next().unwrap();
    let mut number_parts = card_data.split('|');
    let win_numbers = number_parts.next().unwrap();
    let card_numbers = number_parts.next().unwrap();
    let wn: Vec<&str> = win_numbers.split_ascii_whitespace().collect();
    let cn: Vec<&str> = card_numbers.split_ascii_whitespace().collect();
    // Normally use array_tool::vec::Intersect
    let mut val = 0;
    for x in wn {
        for y in &cn {
            if y == &x {
                if val == 0 {
                    val = 1;
                } else {
                    val = val * 2;
                }
                println!("{}:{}", y, val);
            }
        }
    }
    val
}

#[test]
fn test_cards() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    let mut value = 0;
    for line in input.split('\n') {
        value += parse_card(line);
    }
    assert_eq!(value, 13);
}
