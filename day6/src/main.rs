use std::io::Read;
use std::str::Lines;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut lines2 = lines.clone();
    let r = fun_times(&mut lines);
    println!("{}", r);

    // Part 2
    let (time, distance) = parse_2(&mut lines2);
    let res: u64 = run_one_race(&Race { time, distance });
    println!("{}", res);
}

fn fun_times(mut lines: &mut Lines) -> u64 {
    let races = parse(&mut lines);
    let mut wins_count: Vec<u64> = Vec::new();
    for race in &races {
        let wins = run_one_race(race);
        wins_count.push(wins);
    }
    let result: u64 = wins_count.iter().product();
    result
}

fn run_one_race(race: &Race) -> u64 {
    let run_time = 0..race.time;
    let mut wins = 0;
    for run in run_time.collect::<Vec<u64>>() {
        let win = run_race(race.time, run, race.distance);
        if win {
            wins += 1;
        }
    }
    wins
}

fn run_race(time: u64, i: u64, distance: u64) -> bool {
    let run_time = time - i;
    let run_distance = run_time * i;
    if run_distance > distance {
        return true;
    } else {
        return false;
    }
}

fn parse(lines: &mut Lines) -> Vec<Race> {
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();
    let line_one_nums = numbers_in_line(line1);
    let line_two_nums = numbers_in_line(line2);
    let mut races: Vec<Race> = Vec::new();
    for (i, num) in line_one_nums.iter().enumerate() {
        races.push(Race {
            time: line_one_nums[i],
            distance: line_two_nums[i],
        });
    }
    races
}

fn parse_2(lines: &mut Lines) -> (u64, u64) {
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();

    let num_str_one = line1.split(':').collect::<Vec<&str>>()[1];
    let num_str_two = line2.split(':').collect::<Vec<&str>>()[1];

    let nums_str_one: Vec<&str> = num_str_one.split_ascii_whitespace().collect();
    let nums_str_two: Vec<&str> = num_str_two.split_ascii_whitespace().collect();

    let mut s1: String = String::new();
    let mut s2: String = String::new();

    for s in nums_str_one {
        s1.push_str(s);
    }
    for s in nums_str_two {
        s2.push_str(s);
    }

    println!("{}:{}", s1, s2);

    let time: u64 = s1.parse().unwrap();
    let distance: u64 = s2.parse().unwrap();

    (time, distance)
}

fn numbers_in_line(line: &str) -> Vec<u64> {
    let num_str = line.split(':').collect::<Vec<&str>>()[1];
    let num_strs: Vec<&str> = num_str.split_ascii_whitespace().collect();
    let nums: Vec<u64> = num_strs.iter().map(|x| x.parse().unwrap()).collect();
    nums
}

#[cfg(test)]
mod tests {
    use crate::{Race, run_race, fun_times, parse_2, run_one_race};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        let mut lines = INPUT.lines();
        let res = fun_times(&mut lines);
        assert_eq!(288, res);
    }

    #[test]
    fn test_part_2() {
        let mut lines = INPUT.lines();
        let (time, distance) = parse_2(&mut lines);
        assert_eq!(time, 71530);
        assert_eq!(distance, 940200);
        let res = run_one_race(&Race { time, distance });
        assert_eq!(res, 71503);
    }
}