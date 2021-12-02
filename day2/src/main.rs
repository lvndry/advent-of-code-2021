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
struct BasePosition {
    x: i32,
    y: i32,
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
    let directions = content.split('\n');

    let mut part_one_position = BasePosition { x: 0, y: 0 };
    let mut part_two_position = Position { x: 0, y: 0, aim: 0 };

    for instruction in directions {
        let mut split = instruction.split(' ');
        let (direction, step): (Direction, i32) = (
            Direction::from_str(split.next().unwrap()).unwrap(),
            split.next().unwrap().parse().unwrap(),
        );
        // part 1
        match direction {
            Direction::Forward => part_one_position.x += step,
            Direction::Up => part_one_position.y -= step,
            Direction::Down => part_one_position.y += step,
        }

        // part 2
        match direction {
            Direction::Up => part_two_position.aim -= step,
            Direction::Down => part_two_position.aim += step,
            Direction::Forward => {
                part_two_position.x += step;
                part_two_position.y += part_two_position.aim * step
            }
        }
    }

    println!("{}", part_one_position.x * part_one_position.y);
    println!("{}", part_two_position.x * part_two_position.y)
}
