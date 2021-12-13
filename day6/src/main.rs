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
    let values = content.split(',');
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

    println!("{}", fishes.len());
    let p2 = part2(content.clone());
    println!("{}", p2)
}

fn part2(input: std::string::String) -> u64 {
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
