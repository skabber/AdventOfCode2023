use std::{char, io::Read, str::Lines};

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
    None,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    glyph: Glyph,
}

#[derive(Debug, Clone, Copy)]
struct Glyph(Direction, Direction, char);

const NORTH_SOUTH: Glyph = Glyph(Direction::North, Direction::South, '|');
const EAST_WEST: Glyph = Glyph(Direction::East, Direction::West, '-');
const NORTH_EAST: Glyph = Glyph(Direction::North, Direction::East, 'L');
const NORTH_WEST: Glyph = Glyph(Direction::North, Direction::West, 'J');
const SOUTH_WEST: Glyph = Glyph(Direction::South, Direction::West, '7');
const SOUTH_EAST: Glyph = Glyph(Direction::South, Direction::East, 'F');
const START: Glyph = Glyph(Direction::None, Direction::None, 'S');
const LAND: Glyph = Glyph(Direction::None, Direction::None, '.');

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
    let _s = find_start_point(&lines);
}

fn find_start_point(lines: &Lines) -> (usize, usize) {
    let i: Vec<(usize, usize)> = lines
        .clone()
        .enumerate()
        .filter(|(_, l)| l.contains('S'))
        .map(|(i, l)| (l.find('S').unwrap(), i))
        .collect();
    i[0]
}

fn traverse_map_from_start(start: (usize, usize), lines: Lines) -> usize {
    // let x: &str = lines.collect::<Vec<&str>>()[start.1];
    let s = find_next_point(start.0, start.1, lines);
    // println!("{}", char_for_point(start, &lines));
    0
}

fn find_next_point(x: usize, y: usize, lines: Lines) -> (usize, usize) {
    let mut connected_points: Vec<Point> = Vec::new();
    let current_char = point_for_point((Some(x), Some(y)), &lines);

    let top = point_for_point((Some(x), y.checked_sub(1)), &lines);
    let left = point_for_point((x.checked_sub(1), Some(y)), &lines);
    let right = point_for_point((Some(x + 1), Some(y)), &lines);
    let bottom = point_for_point((Some(x), Some(y + 1)), &lines);

    if top.is_some() {
        let tc = top.unwrap();
        let g = char_to_glyph(tc.glyph.2);
        match g.0 {
            Direction::South => connected_points.push(tc),
            _ => (),
        }
        // match g.1 {
        //     Direction::North => connected_points.push(tc),
        //     _ => (),
        // }
    }

    if left.is_some() {
        let lc = left.unwrap();
        let g = char_to_glyph(lc.glyph.2);
        match g.0 {
            Direction::East => connected_points.push(lc),
            _ => (),
        }
        // match g.1 {
        //     Direction::West => connected_points.push(lc),
        //     _ => (),
        // }
    }

    if right.is_some() {
        let rc = right.unwrap();
        let g = char_to_glyph(rc.glyph.2);
        // match g.0 {
        //     Direction::East => connected_points.push(rc),
        //     _ => (),
        // }
        match g.1 {
            Direction::West => connected_points.push(rc),
            _ => (),
        }
    }

    if bottom.is_some() {
        let bc = bottom.unwrap();
        let g = char_to_glyph(bc.glyph.2);
        match g.0 {
            Direction::North => connected_points.push(bc),
            _ => (),
        }
    }

    // let surrounding_chars: Vec<char> = vec![top, right, bottom, left]
    //     .iter()
    //     .filter(|s| s.is_some())
    //     .map(|f| f.unwrap())
    //     .collect();
    // for c in surrounding_chars {
    //     let g = char_to_glyph(&c);
    // }

    println!("Connected Points {:?}", connected_points);

    // println!("{:?} : {:?} : {:?} : {:?}", top, right, bottom, left);
    (0, 0)
}

fn point_for_point(point: (Option<usize>, Option<usize>), lines: &Lines) -> Option<Point> {
    match point.1 {
        Some(x) => {
            let line = lines.to_owned().collect::<Vec<&str>>()[x];
            match point.0 {
                Some(y) => {
                    let c = line.chars().nth(y).unwrap().clone();
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

    const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part1() {
        let lines = INPUT.lines();
        let start_point = find_start_point(&lines);
        println!("{:?}", start_point);
        let mid_point = traverse_map_from_start(start_point, lines);
        assert_eq!(mid_point, 8);
    }
}
