use std::fs;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
struct LanterFish {
    timer: u16,
}

impl LanterFish {
    fn from_str(l: &str) -> Self {
        let timer: u16 = l.parse().unwrap();
        Self { timer }
    }
}

fn main() {
    let input = Path::new("./input.txt");
    let content = fs::read_to_string(input).expect("Unable to read file");
    println!("{}", part_1(&content.clone()));
    println!("{}", part2(&content.clone()));
}

fn part_1(input: &str) -> usize {
    let values = input.split(',');
    let mut fishes: Vec<LanterFish> = Vec::new().to_owned();

    for value in values {
        fishes.push(LanterFish::from_str(value));
    }

    for _ in 0..80 {
        let mut clone: Vec<LanterFish> = Vec::new();
        for mut fish in fishes.clone().into_iter() {
            if fish.timer == 0 {
                fish.timer = 6;
                clone.push(LanterFish { timer: 8 })
            } else {
                fish.timer -= 1;
            }
            clone.push(fish);
        }
        fishes = clone.clone();
    }

    fishes.len()
}

fn part2(input: &str) -> u64 {
    let m: Vec<i64> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut n = [0; 9];
    for v in m {
        n[v as usize] += 1;
    }

    for _ in 0..256 {
        n.rotate_left(1);
        n[6] += n[8];
    }

    n.iter().sum::<u64>()
}
