use std::fs;
use std::path::Path;

/**
 * input: 011111101011
 * rate is the number of zero for every bit in each line
 * for each line we read every bit and add 1 if there is a 0
 * after reading the input rate is [1,0,0,0,0,0,0,1,0,1,0,0]
 * after reading all the lines we can know the dominant bits by checking if more than half of line have zeros or not
 * from that we can build our gamma and epislon values and determine the power comsuption
 * of elements in my input
 */
fn main() {
    let input = Path::new("./input.txt");
    let content = fs::read_to_string(input).expect("Unable to read file");
    let diagnostics: Vec<&str> = content.lines().collect();

    part1(diagnostics.clone());
    part2(diagnostics);
}

fn part1(input: Vec<&str>) {
    let diagnostics = input;
    let mut rate: Vec<u32> = vec![0; diagnostics[0].len()];
    for diagnostic in diagnostics.to_vec() {
        for (i, bit) in diagnostic.chars().enumerate() {
            if bit == '0' {
                rate[i] += 1;
            }
        }
    }
    let (gamma, epsilon) = {
        let mut gamma = "".to_owned();
        let mut epsilon = "".to_owned();
        rate.iter().for_each(|&b| {
            if b > ((diagnostics.len() / 2) as u32) {
                gamma.push('0');
                epsilon.push('1');
            } else {
                gamma.push('1');
                epsilon.push('0');
            }
        });

        (gamma, epsilon)
    };

    let gamma = isize::from_str_radix(&gamma[..], 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon[..], 2).unwrap();
    println!("{}, {}", gamma, epsilon);
    println!("{}", gamma * epsilon);
}

fn part2(input: Vec<&str>) {
    let mut oxygen_values = input;
    let mut oxygen_comparaison = String::from("");

    let mut i = 0;
    while oxygen_values.len() > 1 {
        let mut oxygen_rate: Vec<u32> = vec![0; oxygen_values[0].len()];
        for oxygen_value in oxygen_values.to_vec() {
            for (i, bit) in oxygen_value.chars().enumerate() {
                if bit == '0' {
                    oxygen_rate[i] += 1;
                }
            }
        }
        if oxygen_rate[i] > ((oxygen_values.len() / 2) as u32) {
            oxygen_comparaison.push('0');
        } else {
            oxygen_comparaison.push('1')
        }

        oxygen_values = oxygen_values
            .into_iter()
            .filter(|d| d.starts_with(&oxygen_comparaison))
            .collect();
        i += 1;
    }

    let oxygen_rating = oxygen_values[0];
    let gamma = isize::from_str_radix(oxygen_rating, 2).unwrap();
    let epsilon = gamma & 0xFFF;
    println!("{}, {}", gamma, epsilon);
    println!("{}", gamma * epsilon);
}
