use std::{char, io::Read, str::Lines};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
    Start,
    None,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    glyph: Glyph,
}

#[derive(Debug, Clone, Copy)]
struct Glyph {
    directions: (Direction, Direction),
    c: char,
}

const NORTH_SOUTH: Glyph = Glyph {
    directions: (Direction::North, Direction::South),
    c: '|',
};
const EAST_WEST: Glyph = Glyph {
    directions: (Direction::East, Direction::West),
    c: '-',
};
const NORTH_EAST: Glyph = Glyph {
    directions: (Direction::North, Direction::East),
    c: 'L',
};
const NORTH_WEST: Glyph = Glyph {
    directions: (Direction::North, Direction::West),
    c: 'J',
};
const SOUTH_WEST: Glyph = Glyph {
    directions: (Direction::South, Direction::West),
    c: '7',
};
const SOUTH_EAST: Glyph = Glyph {
    directions: (Direction::South, Direction::East),
    c: 'F',
};
const START: Glyph = Glyph {
    directions: (Direction::Start, Direction::Start),
    c: 'S',
};
const LAND: Glyph = Glyph {
    directions: (Direction::None, Direction::None),
    c: '.',
};

fn char_to_glyph(c: char) -> Glyph {
    match c {
        '|' => NORTH_SOUTH,
        '-' => EAST_WEST,
        'L' => NORTH_EAST,
        'J' => NORTH_WEST,
        '7' => SOUTH_WEST,
        'F' => SOUTH_EAST,
        'S' => START,
        '.' => LAND,
        _ => panic!("Never happens with {}", c),
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let lines = input.lines().clone();
    let start_point = find_start_point(&lines);
    println!("Start Point: {:?}", start_point);
    let mid_point = traverse_map_from_start(start_point, lines) / 2;
    println!("{:?}", mid_point); // 6931
}

fn find_start_point(lines: &Lines) -> Point {
    let i: Vec<(usize, usize)> = lines
        .clone()
        .enumerate()
        .filter(|(_, l)| l.contains('S'))
        .map(|(i, l)| (l.find('S').unwrap(), i))
        .collect();

    Point {
        x: i[0].0,
        y: i[0].1,
        glyph: START,
    }
}

fn traverse_map_from_start(start: Point, lines: Lines) -> usize {
    let mut np = start;
    let mut pp: Option<Point> = None;
    let mut count = 0;
    loop {
        count += 1;
        let rp = find_next_point(np, lines.clone(), pp);
        match rp {
            Some(p) => {
                println!("{}", np.glyph.c);
                pp = Some(np);
                np = p;
            }
            None => {
                println!("No more points");
                break;
            }
        }
    }
    count
}

fn find_next_point(point: Point, lines: Lines, previous_point: Option<Point>) -> Option<Point> {
    let mut connected_points: Vec<Point> = Vec::new();

    let top = point_for_point((Some(point.x), point.y.checked_sub(1)), &lines);
    let left = point_for_point((point.x.checked_sub(1), Some(point.y)), &lines);
    let right = point_for_point((Some(point.x + 1), Some(point.y)), &lines);
    let bottom = point_for_point((Some(point.x), Some(point.y + 1)), &lines);

    if top.is_some() {
        let t = top.unwrap();

        if (t.glyph.directions.1 == Direction::South || t.glyph.directions.0 == Direction::South)
            && (point.glyph.directions.0 == Direction::North
                || point.glyph.directions.1 == Direction::North
                || point.glyph.directions.0 == Direction::Start)
        {
            connected_points.push(t);
        }
    }
    if right.is_some() {
        let t = right.unwrap();
        if (t.glyph.directions.1 == Direction::West || t.glyph.directions.0 == Direction::West)
            && (point.glyph.directions.0 == Direction::East
                || point.glyph.directions.1 == Direction::East
                || point.glyph.directions.0 == Direction::Start)
        {
            connected_points.push(t)
        }
    }

    if bottom.is_some() {
        let t = bottom.unwrap();
        if (t.glyph.directions.1 == Direction::North || t.glyph.directions.0 == Direction::North)
            && (point.glyph.directions.0 == Direction::South
                || point.glyph.directions.1 == Direction::South
                || point.glyph.directions.0 == Direction::Start)
        {
            connected_points.push(t)
        }
    }
    if left.is_some() {
        let t = left.unwrap();
        if (t.glyph.directions.1 == Direction::East || t.glyph.directions.0 == Direction::East)
            && (point.glyph.directions.0 == Direction::West
                || point.glyph.directions.1 == Direction::West
                || point.glyph.directions.0 == Direction::Start)
        {
            connected_points.push(t)
        }
    }

    match previous_point {
        Some(pp) => {
            let p: Vec<&Point> = connected_points
                .iter()
                .filter(|cp| {
                    let z = **cp;
                    z.x != pp.x || z.y != pp.y
                })
                .collect();
            if p.len() == 1 && p[0].glyph.c == 'S' {
                Some(p[0].to_owned())
            } else if p.is_empty() {
                None
            } else {
                Some(
                    *p.iter()
                        .find(|i| i.glyph.directions.0 != Direction::Start)
                        .unwrap()
                        .to_owned(),
                )
            }
        }
        None => Some(connected_points[0]),
    }
}

fn point_for_point(point: (Option<usize>, Option<usize>), lines: &Lines) -> Option<Point> {
    match point.1 {
        Some(x) => {
            if x >= lines.to_owned().collect::<Vec<&str>>().len() {
                return None;
            }
            let line = lines.to_owned().collect::<Vec<&str>>()[x];
            match point.0 {
                Some(y) => {
                    let len = line.chars().collect::<Vec<char>>().len();
                    if y >= len {
                        return None;
                    }
                    let c = line.chars().nth(y).unwrap();
                    let g = char_to_glyph(c);
                    Some(Point {
                        x: point.0.unwrap(),
                        y: point.1.unwrap(),
                        glyph: g,
                    })
                }
                None => None,
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_start_point, traverse_map_from_start};

    const INPUT1: &str = "
F-7F---7
|.|||J||
|.LJJ.L|
|..F-S-J
L--J....";

    #[test]
    fn test_part1() {
        let lines = INPUT1.lines();
        let start_point = find_start_point(&lines);
        let mid_point = traverse_map_from_start(start_point, lines) / 2;
        assert_eq!(mid_point, 13);
    }

    const INPUT2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part2() {
        let lines = INPUT2.lines();
        let start_point = find_start_point(&lines);
        let mid_point = traverse_map_from_start(start_point, lines) / 2;
        assert_eq!(mid_point, 80);
    }
}
