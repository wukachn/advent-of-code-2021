use std::fs;
use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(()),
        }
    }
}

fn follow_instructions(instructions: &[(Direction, i64)], is_part_one: bool) -> (i64, i64) {
    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;
    for (direction, distance) in instructions {
        match direction {
            Direction::Up => {
                if is_part_one {
                    depth -= distance;
                } else {
                    aim -= distance;
                }
            }
            Direction::Down => {
                if is_part_one {
                    depth += distance;
                } else {
                    aim += distance;
                }
            }
            Direction::Forward => {
                x += distance;
                if !is_part_one {
                    depth += aim * distance;
                }
            }
        }
    }
    (x, depth)
}

fn parse_instructions(contents: &str) -> Vec<(Direction, i64)> {
    contents
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|chunk| {
            let direction: Direction = chunk[0].parse().unwrap();
            let distance: i64 = chunk[1].parse().unwrap();
            (direction, distance)
        })
        .collect()
}

pub fn solve() {
    let contents: String = fs::read_to_string("inputs/day_02.txt").expect("Unable to read file!");
    let instructions: Vec<(Direction, i64)> = parse_instructions(&contents);

    println!("🦀 Day 2 🦀");
    let part_one_coords = follow_instructions(&instructions, true);
    println!("🦀 Ex 1: {} 🦀", part_one_coords.0 * part_one_coords.1); // 1654760
    let part_one_coords = follow_instructions(&instructions, false);
    println!("🦀 Ex 2: {} 🦀", part_one_coords.0 * part_one_coords.1); // 1956047400
}
