use std::{
    env,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let lines = read_lines().expect("Unable to read lines");
    let rotations: Vec<Rotation> = lines
        .map(|line| Rotation::from_str(&line.unwrap()).unwrap())
        .collect();
    let mut safe = Safe::new();
    let mut result = 0;
    for rotation in rotations {
        safe.rotate(rotation);
        if safe.dial == 0 {
            result += 1;
        }
    }
    println!("Part 1: {}", result);
}

enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    distance: usize,
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s.chars().next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => anyhow::bail!("Invalid direction"),
        };

        let distance = s
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        Ok(Rotation {
            direction,
            distance,
        })
    }
}

struct Safe {
    dial: usize,
}

impl Safe {
    fn new() -> Self {
        Self { dial: 50 }
    }

    fn rotate(&mut self, rotation: Rotation) {
        let adjusted_distance = rotation.distance % 100;

        match rotation.direction {
            Direction::Left => {
                if self.dial < adjusted_distance {
                    self.dial = 100 - (adjusted_distance - self.dial);
                } else {
                    self.dial -= adjusted_distance;
                }
            }

            Direction::Right => self.dial = (self.dial + adjusted_distance) % 100,
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
