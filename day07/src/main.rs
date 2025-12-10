use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines: Vec<String> = read_lines().unwrap().map(|line| line.unwrap()).collect();
    let mut rays = HashSet::new();
    rays.insert(
        lines
            .first()
            .unwrap()
            .chars()
            .enumerate()
            .find(|(_, c)| *c == 'S')
            .unwrap()
            .0,
    );

    let mut splits = 0;

    for manifold_line in lines.iter().skip(1) {
        for (i, manifold_char) in manifold_line.chars().enumerate() {
            if manifold_char == '^' {
                if rays.contains(&i) {
                    splits += 1;
                    rays.remove(&i);
                    rays.insert(i - 1);
                    rays.insert(i + 1);
                }
            }
        }
    }

    println!("Part 1: {}", splits);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
