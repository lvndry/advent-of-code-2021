use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err("Invalid value".to_string()),
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

fn main() {
    let input = Path::new("./input.txt");
    let content = fs::read_to_string(input).expect("Unable to read file");
    let directions = content.split('\n').collect::<Vec<&str>>();

    part_1(directions.clone());
    part_2(directions.clone());

    println!("{}", part_1(directions.clone()));
    println!("{}", part_1(directions.clone()));
}

fn part_1(directions: Vec<&str>) -> i32 {
    let mut position = Position { x: 0, y: 0, aim: 0 };
    for instruction in directions {
        let mut split = instruction.split(' ');
        let (direction, step): (Direction, i32) = (
            Direction::from_str(split.next().unwrap()).unwrap(),
            split.next().unwrap().parse().unwrap(),
        );
        match direction {
            Direction::Forward => position.x += step,
            Direction::Up => position.y -= step,
            Direction::Down => position.y += step,
        }
    }

    position.x * position.y
}

fn part_2(directions: Vec<&str>) -> i32 {
    let mut position = Position { x: 0, y: 0, aim: 0 };
    for instruction in directions {
        let mut split = instruction.split(' ');
        let (direction, step): (Direction, i32) = (
            Direction::from_str(split.next().unwrap()).unwrap(),
            split.next().unwrap().parse().unwrap(),
        );
        match direction {
            Direction::Up => position.aim -= step,
            Direction::Down => position.aim += step,
            Direction::Forward => {
                position.x += step;
                position.y += position.aim * step
            }
        }
    }

    position.x * position.y
}
