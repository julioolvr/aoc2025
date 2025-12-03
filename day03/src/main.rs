use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines().expect("Unable to read lines");
    let mut part_1 = 0;

    for line in lines {
        let line = line.unwrap();
        let mut max_digit = line.chars().nth(0).unwrap().to_digit(10).unwrap();
        let mut max_index = 0;

        for (i, c) in line.chars().take(line.len() - 1).skip(1).enumerate() {
            let digit = c.to_digit(10).unwrap();
            if digit > max_digit {
                max_digit = digit;
                max_index = i + 1;
            }
        }

        let min_digit = line
            .chars()
            .skip(max_index + 1)
            .map(|c| c.to_digit(10).unwrap())
            .max()
            .unwrap();

        part_1 += max_digit * 10 + min_digit;
    }

    println!("Part 1: {}", part_1);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
