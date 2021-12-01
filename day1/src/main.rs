use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("./input.txt");
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");
    let depths: Vec<i32> = contents.split('\n').map(|n| n.parse().unwrap()).collect();
    // part 1
    let increased_len = depths
        .windows(2)
        .fold(0, |acc, val| if val[1] > val[0] { acc + 1 } else { acc });

    println!("{}", increased_len);

    // part 2
    let mut prev_sum = depths[0] + depths[1] + depths[2];
    let mut moving_count = 0;

    for i in 1..depths.len() - 2 {
        let current_sum = depths[i] + depths[i + 1] + depths[i + 2];
        if current_sum > prev_sum {
            moving_count += 1
        }

        prev_sum = current_sum
    }

    println!("{}", moving_count);
}
