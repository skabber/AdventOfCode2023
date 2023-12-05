use std::{collections::HashSet, io::BufRead};

fn main() {
    let mut val = 0;
    let mut copies: Vec<usize> = Vec::new();

    for (index, line) in std::io::stdin().lock().lines().enumerate() {
        let count = parse_card(line.unwrap().as_str());
        if copies.len() <= index {
            copies.push(1);
        } else {
            copies[index] += 1;
        }

        let base: u32 = 2;
        if count > 0 {
            val += base.pow(count as u32 - 1);
        }

        for j in 0..count {
            let next_index = index + 1 + j;
            if copies.len() <= next_index {
                copies.push(copies[index]);
            } else {
                copies[next_index] += copies[index];
            }
        }
    }

    let card_count = copies.iter().sum::<usize>();

    println!("score: {}", val);
    println!("card count: {}", card_count);
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

    let hs1: HashSet<_> = wn.into_iter().collect();
    let hs2: HashSet<_> = cn.into_iter().collect();
    let intersection: HashSet<_> = hs1.intersection(&hs2).collect();
    let result_vec: Vec<_> = intersection.into_iter().cloned().collect();

    result_vec.len()
}

#[test]
fn test_cards() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 1 2 3 4 5 | 6 7 8 9 10
    Card 6: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 7: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    let mut val = 0;
    let mut copies: Vec<usize> = Vec::new();

    for (index, line) in input.split('\n').enumerate() {
        let count = parse_card(line);
        if copies.len() <= index {
            copies.push(1);
        } else {
            copies[index] += 1;
        }

        let base: u32 = 2;
        if count > 0 {
            val += base.pow(count as u32 - 1);
        }

        for j in 0..count {
            let next_index = index + 1 + j;
            if copies.len() <= next_index {
                copies.push(copies[index]);
            } else {
                copies[next_index] += copies[index];
            }
        }
    }

    println!("{:?}", copies);

    let card_count = copies.iter().sum::<usize>();
    assert_eq!(val, 13);
    assert_eq!(card_count, 31);
}
