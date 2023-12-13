use std::{io::Read, str::Lines};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let lines = input.lines().clone();
    let sets = parse_line(lines);
    let answers: Vec<i32> = sets.iter().map(|s| solve_line(s)).collect();
    let sum: i32 = answers.iter().sum();
    println!("{:?}", sum); // 1789635132
}

fn solve_line(input: &[i32]) -> i32 {
    let mut nexts: Vec<Vec<i32>> = vec![input.to_vec()];
    let mut count = 0;
    loop {
        let mut nums: Vec<i32> = Vec::new();
        for (i, val) in nexts[count].iter().enumerate() {
            if i > 0 {
                let x = nexts[count][i - 1];
                let y = val;
                let z = y - x;
                nums.push(z);
            }
        }
        count += 1;
        nexts.push(nums.clone());
        if nums.iter().all(|f| f == &0) {
            break;
        }
    }
    nexts.reverse();
    let mut prev_set: Vec<i32> = Vec::new();
    for (i, s) in nexts.iter().enumerate() {
        let mut y = s.clone();
        if i == 0 {
            y.push(0);
        } else {
            let value = y[y.len() - 1] + prev_set[prev_set.len() - 1];
            y.push(value);
        }
        prev_set = y;
    }
    prev_set[prev_set.len() - 1]
}

fn parse_line(lines: Lines) -> Vec<Vec<i32>> {
    let mut sets: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        sets.push(
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        );
    }
    sets
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        let lines = INPUT.lines();
        let sets = parse_line(lines);
        let answers: Vec<i32> = sets.iter().map(|s| solve_line(s)).collect();
        let sum: i32 = answers.iter().sum();
        println!("{:?}", sum);
        assert_eq!(sum, 114);
    }
}
