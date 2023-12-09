use std::io::Read;
use std::str::Lines;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let r = fun_times(&mut lines);
    println!("{}", r);
}

fn fun_times(mut lines: &mut Lines) -> u32 {
    let races = parse(&mut lines);
    let mut wins_count: Vec<u32> = Vec::new();
    for race in &races {
        let run_time = 0..race.time;
        let mut wins = 0;
        for run in run_time.collect::<Vec<u32>>() {
            let win = run_race(race.time, run, race.distance);
            if win {
                wins += 1;
            }
        }
        wins_count.push(wins);
    }
    let result: u32 = wins_count.iter().product();
    result
}

fn run_race(time: u32, i: u32, distance: u32) -> bool {
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

fn numbers_in_line(line: &str) -> Vec<u32> {
    let num_str = line.split(':').collect::<Vec<&str>>()[1];
    let num_strs: Vec<&str> = num_str.split_ascii_whitespace().collect();
    let nums: Vec<u32> = num_strs.iter().map(|x| x.parse().unwrap()).collect();
    nums
}

#[cfg(test)]
mod tests {
    use std::str::Lines;
    use crate::{parse, run_race, fun_times};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_race() {
        let mut lines = INPUT.lines();
        let res = fun_times(&mut lines);
        assert_eq!(288, res);
    }
}