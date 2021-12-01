use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("./input.txt");
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");
    let depths: Vec<u32> = contents.split('\n').map(|n| n.parse().unwrap()).collect();
    // part 1
    let increased_depth_count =
        depths
            .windows(2)
            .fold(0, |acc, val| if val[1] > val[0] { acc + 1 } else { acc });

    println!("{}", increased_depth_count);

    // part 2
    let mut iter_window = depths.windows(3);
    let mut prev_sum: u32 = iter_window.next().unwrap().iter().sum();
    let mut increasead_sum_count = 0;
    for (_, val) in iter_window.enumerate() {
        let current_sum: u32 = val.iter().sum();
        if current_sum > prev_sum {
            increasead_sum_count += 1;
        }

        prev_sum = current_sum
    }

    println!("{}", increasead_sum_count);
}
