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
    let mut part_1 = 0;
    for rotation in rotations {
        safe.rotate(rotation);
        if safe.dial == 0 {
            part_1 += 1;
        }
    }
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", safe.times_through_zero);
}

enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    distance: usize,
}

impl Rotation {
    fn value(&self) -> isize {
        match self.direction {
            Direction::Left => -(self.distance as isize),
            Direction::Right => self.distance as isize,
        }
    }
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
    times_through_zero: usize,
}

impl Safe {
    fn new() -> Self {
        Self {
            dial: 50,
            times_through_zero: 0,
        }
    }

    fn rotate(&mut self, rotation: Rotation) {
        let new_value: isize = self.dial as isize + rotation.value();

        if new_value > 0 {
            self.dial = (new_value % 100) as usize;
            self.times_through_zero += (new_value / 100) as usize;
        } else if new_value == 0 {
            self.dial = 0;
            self.times_through_zero += 1;
        } else {
            self.times_through_zero += (new_value.abs() / 100) as usize;
            if self.dial != 0 {
                self.times_through_zero += 1;
            }
            self.dial = (100 - (new_value % 100).abs()) as usize % 100;
        }
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
