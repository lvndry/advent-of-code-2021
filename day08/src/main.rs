use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("./input.txt");
    let content = fs::read_to_string(input).expect("Unable to read file");
    part_1(&content);
}

fn part_1(content: &str) {
    let lines: Vec<&str> = content.split('\n').collect();
    let digits = lines
        .iter()
        .map(|l| {
            l.split('|')
                .next_back()
                .unwrap()
                .trim()
                .split(' ')
                .collect::<Vec<&str>>()
        })
        .flatten();

    let num = digits.into_iter().fold(0, |acc, d| {
        if [2, 3, 4, 7].contains(&d.len()) {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", num);
}
