use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines: Vec<String> = read_lines()
        .expect("Unable to read lines")
        .map(|line| line.unwrap())
        .collect();
    let part_1: usize = lines
        .iter()
        .map(|batteries| highest_joltage(&batteries, 2))
        .sum();

    println!("Part 1: {}", part_1);

    let part_2: usize = lines
        .iter()
        .map(|batteries| highest_joltage(&batteries, 12))
        .sum();

    println!("Part 2: {}", part_2);
}

fn highest_joltage(batteries: &str, batteries_count: usize) -> usize {
    let mut result: usize = 0;
    let mut last_index = 0;

    for n in 0..batteries_count {
        let mut max: usize = 0;

        for i in last_index..=(batteries.len() - batteries_count + n) {
            let digit = batteries.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
            if digit > max {
                max = digit;
                last_index = i + 1;
            }
        }

        result *= 10;
        result += max;
    }

    return result;
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
