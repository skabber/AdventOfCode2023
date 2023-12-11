use std::str::Lines;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
}

fn card_value(card: &char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("{:?}:{:?}", self.cards, other.cards);
        let mut o = self.hand_value().cmp(&other.hand_value());
        // println!("{:?} is {:?} than {:?}", self, o, other);
        if o == Ordering::Equal {
            o = self.cards.cmp(&other.cards);
        }
        o
    }
}

impl Hand {
    fn hand_value(&self) -> u32 {
        let mut card_map: HashMap<&char, u32> = HashMap::new();
        self.cards.iter().for_each(|c| {
            match card_map.get(c) {
                None => { card_map.insert(c, 1); }
                Some(_) => { *card_map.get_mut(c).unwrap() += 1; }
            }
        });

        let mut val = 0;
        if card_map.keys().len() == 1 {
            val = 5; // 5 of a kind
        } else if card_map.len() == 5 {
            val = 0; // no pairs
        } else if card_map.len() == 2 && card_map.values().all(|v| *v == 2 || *v == 3) {
            val = 3; // full house
        } else if card_map.len() == 4 {
            val = 1; // one pair
        } else if card_map.len() == 3 {
            val = 3; // two pair
        }
        val
    }
}

fn main() {
    println!("Hello, world!");
}

fn parse_lines(lines: Lines) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        hands.push(parse_line(line));
    }
    hands
}

fn parse_line(line: &str) -> Hand {
    let line_parts: Vec<&str> = line.split_ascii_whitespace().collect();
    let hand_str: &str = line_parts[0];
    let bid: u64 = (line_parts[1]).parse().unwrap();
    Hand {
        cards: hand_str.chars().collect(),
        bid,
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_lines;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        let lines = INPUT.lines();
        let mut hands = parse_lines(lines);
        hands.sort();
        println!("{:?}", hands);

        let mut res = 0;
        for (i, hand) in hands.iter().enumerate() {
            let val = (i + 1) as u64 * hand.bid;
            res += val;
        }

        println!("Result: {}", res);
        println!("{:?}", hands);
        assert_eq!(6440, res);
    }
}