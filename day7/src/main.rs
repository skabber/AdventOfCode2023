use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;
use std::str::Lines;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    card_values: Vec<u8>,
    bid: u64,
    with_joker: bool,
}

fn card_value(card: &char, with_joker: bool) -> u8 {
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
        'J' => {
            if with_joker {
                1
            } else {
                11
            }
        }
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

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut o = self.hand_value().cmp(&other.hand_value());
        if o == Ordering::Equal {
            o = self.card_values.cmp(&other.card_values);
        }
        o
    }
}

impl Hand {
    fn hand_value(&self) -> u32 {
        let mut card_map: HashMap<&char, u32> = HashMap::new();
        let mut joker_count = 0;
        self.cards.iter().for_each(|c| match card_map.get(c) {
            None => {
                if self.with_joker {
                    if c != &'J' {
                        card_map.insert(c, 1);
                    } else {
                        joker_count += 1;
                    }
                } else {
                    card_map.insert(c, 1);
                }
            }
            Some(_) => {
                *card_map.get_mut(c).unwrap() += 1;
            }
        });

        let mut clone = card_map.clone();

        let max = match clone.values().max() {
            None => {
                card_map.insert(&'J', 5);
                0
            }
            Some(x) => *x,
        };

        clone = card_map.clone();

        for key in clone.keys() {
            if card_map[key] == max {
                *card_map.get_mut(key).unwrap() += joker_count;
                break;
            }
        }

        let mut val = 0;
        if card_map.keys().len() == 1 {
            val = 6; // 5 of a kind
        } else if card_map.len() == 2 && card_map.values().max().unwrap() == &4 {
            val = 5; // four of a kind
        } else if card_map.len() == 2 && card_map.values().max().unwrap() == &3 {
            val = 4; // full house
        } else if card_map.len() == 3 && card_map.values().max().unwrap() == &3 {
            val = 3; // three of a kind
        } else if card_map.len() == 3 && card_map.values().max().unwrap() == &2 {
            val = 2; // two pair
        } else if card_map.len() == 4 {
            val = 1; // one pair
        } else if card_map.len() == 5 {
            val = 0; // no pairs
        }
        val
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let lines = input.lines().clone();
    let mut hands = parse_lines(lines, false);
    hands.sort();
    let sum = sum_hands(&mut hands); // 249726565
    println!("{}", sum);
    let mut hands = parse_lines(input.lines(), true);
    hands.sort();
    let sum = sum_hands(&mut hands); // 251135960
    println!("{}", sum);
}

fn sum_hands(hands: &mut [Hand]) -> u64 {
    hands.sort();

    let mut res = 0;
    for (i, hand) in hands.iter().enumerate() {
        let val = (i + 1) as u64 * hand.bid;
        res += val;
    }
    res
}

fn parse_lines(lines: Lines, with_jokers: bool) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        hands.push(parse_line(line, with_jokers));
    }
    hands
}

fn parse_line(line: &str, with_jokers: bool) -> Hand {
    let line_parts: Vec<&str> = line.split_ascii_whitespace().collect();
    let hand_str: &str = line_parts[0];
    let bid: u64 = line_parts[1].parse().unwrap();
    Hand {
        cards: hand_str.chars().collect(),
        card_values: hand_str
            .chars()
            .map(|c| card_value(&c, with_jokers))
            .collect(),
        bid,251135960
#[cfg(test)]
mod tests {
    use crate::{parse_lines, sum_hands};

    const INPUT: &str = "32T3K 765   
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        let lines = INPUT.lines();
        let mut hands = parse_lines(lines, false);

        let res = sum_hands(&mut hands);

        println!("Result: {}", res);
        assert_eq!(6440, res);
    }

    #[test]
    fn test_part2() {
        let lines = INPUT.lines();
        let mut hands = parse_lines(lines, true);

        let res = sum_hands(&mut hands);

        println!("Result: {}", res);
        assert_eq!(5905, res);
    }
}
