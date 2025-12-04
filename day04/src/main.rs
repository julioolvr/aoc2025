use std::collections::HashSet;
use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines: Vec<String> = read_lines().unwrap().map(|line| line.unwrap()).collect();
    let mut coordinates: HashSet<(isize, isize)> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '@' {
                coordinates.insert((x as isize, y as isize));
            }
        }
    }

    let mut part_1 = 0;

    for (x, y) in coordinates.iter() {
        let mut adjacents = 0;

        if coordinates.contains(&(x - 1, y - 1)) {
            adjacents += 1;
        }
        if coordinates.contains(&(*x, y - 1)) {
            adjacents += 1;
        }
        if coordinates.contains(&(x + 1, y - 1)) {
            adjacents += 1;
        }
        if coordinates.contains(&(x - 1, *y)) {
            adjacents += 1;
        }
        if coordinates.contains(&(x + 1, *y)) {
            adjacents += 1;
        }
        if coordinates.contains(&(x - 1, y + 1)) {
            adjacents += 1;
        }
        if coordinates.contains(&(*x, y + 1)) {
            adjacents += 1;
        }
        if coordinates.contains(&(x + 1, y + 1)) {
            adjacents += 1;
        }

        if adjacents < 4 {
            part_1 += 1;
        }
    }

    println!("Part 1: {}", part_1);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
