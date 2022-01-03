use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("./input.txt");
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");
    let depths: Vec<u32> = contents.split('\n').map(|n| n.parse().unwrap()).collect();
    let part_1 = part_1(depths.clone());
    println!("{}", part_1);
    let part_2 = part_2(depths.clone());
    println!("{}", part_2);
}

fn part_1(depths: Vec<u32>) -> usize {
    depths.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part_2(depths: Vec<u32>) -> usize {
    depths.windows(4).filter(|w| w[3] > w[0]).count()
}
