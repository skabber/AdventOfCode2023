use std::io::BufRead;

// answer 55712
fn main() {
    part2();
}

fn _part1() {
    let mut sum = 0;
    for line in std::io::stdin().lock().lines() {
        let mut first_num: Option<u32> = None;
        let mut last_num: Option<u32> = None;
        for ch in line.unwrap().chars() {
            if ch.is_ascii_digit() {
                match first_num {
                    None => first_num = Some(ch.to_digit(10).unwrap()),
                    Some(_) => last_num = Some(ch.to_digit(10).unwrap()),
                }
            }
        }
        if last_num.is_none() {
            last_num = Some(first_num.unwrap());
        }

        let num_str = format!("{}{}", first_num.unwrap(), last_num.unwrap());
        let x = num_str.parse::<u32>().unwrap();

        sum += x;
    }
    println!("{}", sum);
}

#[derive(Debug, Copy, Clone)]
struct Item {
    index: usize,
    value: u32,
}
// answer 55413
fn part2() {
    let mut sum = 0;
    for line in std::io::stdin().lock().lines() {
        let x = parseline(line.unwrap());

        sum += x;
    }
    println!("{}", sum);
}

fn parseline(l: String) -> u32 {
    let mut first_num: Option<Item> = None;
    let mut last_num: Option<Item> = None;

    // digits
    for (i, ch) in l.chars().enumerate() {
        if ch.is_ascii_digit() {
            match first_num {
                None => {
                    first_num = Some(Item {
                        index: i,
                        value: ch.to_digit(10).unwrap(),
                    })
                }
                Some(_) => {
                    last_num = Some(Item {
                        index: i,
                        value: ch.to_digit(10).unwrap(),
                    })
                }
            }
        }
    }
    if last_num.is_none() {
        last_num = Some(Item {
            index: first_num.as_ref().unwrap().index,
            value: first_num.as_ref().unwrap().value,
        });
    }

    // words
    let num_list = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut first_word: Option<Item> = None;
    let mut last_word: Option<Item> = None;
    let mut found_words: Vec<Item> = Vec::new();
    for i in num_list {
        if l.contains(i) {
            let ind = l.find(i).unwrap();
            let rind = l.rfind(i).unwrap();
            if ind != rind {
                found_words.push(Item {
                    index: l.find(i).unwrap(),
                    value: word2digit(i),
                });
                found_words.push(Item {
                    index: l.rfind(i).unwrap(),
                    value: word2digit(i),
                });
            } else {
                found_words.push(Item {
                    index: l.find(i).unwrap(),
                    value: word2digit(i),
                });
            }
        }
    }

    match found_words.len() {
        1 => first_word = Some(*found_words.first().unwrap()),
        r => {
            if r > 1 {
                found_words.sort_by_key(|d| d.index);
                first_word = Some(*found_words.first().unwrap());
                last_word = Some(*found_words.last().unwrap());
            }
        }
    }

    if last_word.is_none() && first_word.is_some() {
        last_word = first_word;
    }

    let mut first_item = first_num;
    let mut last_item = last_num;
    if first_word.is_some() && first_word.as_ref().unwrap().index < first_item.unwrap().index {
        first_item = first_word;
    }
    if last_word.is_some() && last_word.as_ref().unwrap().index > last_item.unwrap().index {
        last_item = last_word;
    }

    let num_str = format!("{}{}", first_item.unwrap().value, last_item.unwrap().value);
    println!(
        "{} : {} : {} : {}",
        l,
        first_item.unwrap().value,
        last_item.unwrap().value,
        num_str
    );
    num_str.parse::<u32>().unwrap()
}

pub fn word2digit(w: &str) -> u32 {
    match w {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::parseline;

    #[test]
    fn it_works() {
        let input = String::from("1rsjbbhtkbbfourqzdhlone4eighttwo");
        let result = parseline(input);
        assert_eq!(result, 12);
    }

    #[test]
    fn fine() {
        let input = String::from("3two3eightjszbfourkxbh5twonepr");
        let result = parseline(input.clone());
        assert_eq!(result, 31, "{}", input);
    }

    #[test]
    fn ugh() {
        let input = String::from("bnjpqcqdzmeight2gtjhqeight");
        let result = parseline(input);
        assert_eq!(result, 88);
    }
}
