use std::io::BufRead;

struct AvailableCubeItem {
    color: String,
    count: u32,
}

struct GameRun {
    id: u32,
    sets: Vec<Set>,
}

struct Set {
    cubes: Vec<AvailableCubeItem>,
}

fn main() {
    let mut id_sum = 0;
    let mut powers = 0;
    for line in std::io::stdin().lock().lines() {
        let run = parseline(line.unwrap());
        if game_is_possible(&run) {
            println!("Game is possible.");
            id_sum += run.id;
        }
        let values = fewest_cubes_possible(run);
        powers += values.iter().copied().reduce(|a, b| a * b).unwrap();
    }
    println!("{}", id_sum);
    println!("{}", powers);
}

fn parseline(l: String) -> GameRun {
    println!("{}", l);
    let mut first_parts = l.split(':');
    let game_num = first_parts.next();
    let game_num_parts: Vec<&str> = game_num.unwrap().split_whitespace().collect();
    let game_num_val: u32 = game_num_parts.last().unwrap().trim().parse().unwrap();
    let game_data = first_parts.next();
    let game_data_parts = game_data.unwrap().split(';');
    let mut game_sets: Vec<Set> = Vec::new();
    for set_item in game_data_parts {
        let set_set_parts = set_item.split(',');
        let mut cubes: Vec<AvailableCubeItem> = Vec::new();
        for game_item in set_set_parts {
            let game_item_parts: Vec<&str> = game_item.split_whitespace().collect();
            let part_num_val: u32 = game_item_parts.first().unwrap().trim().parse().unwrap();
            let part_num_color = game_item_parts.last().unwrap().trim();
            cubes.push(AvailableCubeItem {
                color: part_num_color.to_string(),
                count: part_num_val,
            });
        }
        let s = Set { cubes };
        game_sets.push(s);
    }

    GameRun {
        id: game_num_val,
        sets: game_sets,
    }
}

fn game_is_possible(run: &GameRun) -> bool {
    let available_cubes: [AvailableCubeItem; 3] = [
        AvailableCubeItem {
            color: String::from("red"),
            count: 12,
        },
        AvailableCubeItem {
            color: "green".to_string(),
            count: 13,
        },
        AvailableCubeItem {
            color: "blue".to_string(),
            count: 14,
        },
    ];
    let mut is_possible = true;
    for s in &run.sets {
        for cube in &s.cubes {
            for ac in &available_cubes {
                if cube.color == ac.color && cube.count > ac.count {
                    is_possible = false;
                    println!(
                        "Game not possible. {}:{}>{}",
                        cube.color, cube.count, ac.count
                    );
                    // break;
                }
            }
        }
    }
    is_possible
}

fn fewest_cubes_possible(game_run: GameRun) -> [u32; 3] {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for set in game_run.sets {
        for cube in set.cubes {
            match cube.color.as_str() {
                "red" => {
                    if cube.count > red {
                        red = cube.count
                    }
                }
                "green" => {
                    if cube.count > green {
                        green = cube.count
                    }
                }
                "blue" => {
                    if cube.count > blue {
                        blue = cube.count
                    }
                }
                _ => println!("not red green or blue"),
            }
        }
    }
    [red, green, blue]
}

#[test]
fn line_color_test() {
    // Set 1
    //  2 blue, 3 red
    //  3 green, 3 blue, 6 red
    //  2 green, 2 blue, 9 red
    //  2 red, 4 blue
    let input = "Game 1: 2 blue, 3 red; 3 green, 3 blue, 6 red; 4 blue, 6 red; 2 green, 2 blue, 9 red; 2 red, 4 blue".to_string();
    let result = parseline(input);
    let is_possible = game_is_possible(&result);

    assert!(is_possible);
}

#[test]
fn power_test() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string();
    let game_run = parseline(input);
    let values = fewest_cubes_possible(game_run);
    let power: u32 = values.iter().copied().reduce(|a, b| a * b).unwrap();

    assert_eq!(power, 48);
}
