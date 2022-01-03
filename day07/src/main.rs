use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("./input.txt");
    let content = fs::read_to_string(input).expect("Unable to read file");
    part_1(&content.clone());
    part_2(&content.clone());
}

fn part_1(input: &str) -> i64 {
    let mut values: Vec<i64> = input.split(',').map(|v| v.parse().unwrap()).collect();
    values.sort_unstable();
    let median = values[values.len() / 2];
    let mut fuel: i64 = 0;
    for value in values {
        fuel += (median - value).abs()
    }

    fuel
}

fn part_2(input: &str) -> i64 {
    let values: Vec<i64> = input.split(',').map(|v| v.parse().unwrap()).collect();
    let avg: f64 = values.iter().sum::<i64>() as f64 / values.len() as f64;
    let avg = avg.floor() as i64;
    let v0 = values
        .iter()
        .map(|v| (v - avg).abs())
        .map(|v| v * (v + 1) / 2)
        .sum::<i64>();
    let v1 = values
        .iter()
        .map(|v| (v - avg - 1).abs())
        .map(|v| v * (v + 1) / 2)
        .sum::<i64>();

    v0.min(v1)
}
