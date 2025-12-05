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

    let part_1 = find_removables(&coordinates).len();
    println!("Part 1: {}", part_1);

    let mut part_2 = 0;

    loop {
        let removables = find_removables(&coordinates);

        if removables.len() == 0 {
            break;
        }

        part_2 += removables.len();
        for coordinate in removables {
            coordinates.remove(&coordinate);
        }
    }

    println!("Part 2: {}", part_2);
}

fn find_removables(rolls: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut removables = HashSet::new();

    for (x, y) in rolls {
        let mut adjacents = 0;

        if rolls.contains(&(x - 1, y - 1)) {
            adjacents += 1;
        }
        if rolls.contains(&(*x, y - 1)) {
            adjacents += 1;
        }
        if rolls.contains(&(x + 1, y - 1)) {
            adjacents += 1;
        }
        if rolls.contains(&(x - 1, *y)) {
            adjacents += 1;
        }
        if rolls.contains(&(x + 1, *y)) {
            adjacents += 1;
        }
        if rolls.contains(&(x - 1, y + 1)) {
            adjacents += 1;
        }
        if rolls.contains(&(*x, y + 1)) {
            adjacents += 1;
        }
        if rolls.contains(&(x + 1, y + 1)) {
            adjacents += 1;
        }

        if adjacents < 4 {
            removables.insert((*x, *y));
        }
    }

    return removables
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
