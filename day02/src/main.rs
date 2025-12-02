use std::{
    env,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let line = read_lines()
        .expect("Unable to read lines")
        .next()
        .unwrap()
        .unwrap();
    let ranges: Vec<Range> = line
        .split(',')
        .map(|range_str| Range::from_str(range_str).unwrap())
        .collect();

    let part_1: usize = ranges.iter().map(|range| range.invalid_sum()).sum();

    println!("Part 1: {}", part_1);
}

struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    fn invalid_sum(&self) -> usize {
        let from_digits = self.from.to_string().len();
        let to_digits = self.to.to_string().len();

        let start = if from_digits % 2 == 0 {
            let first_half = self.from / (10_usize.pow(from_digits as u32 / 2));
            let second_half = self.from % (10_usize.pow(from_digits as u32 / 2));

            if second_half <= first_half {
                first_half
            } else {
                first_half + 1
            }
        } else {
            10_usize.pow((from_digits as u32 - 1) / 2)
        };

        let end = if to_digits % 2 == 0 {
            let first_half = self.to / (10_usize.pow(to_digits as u32 / 2)); // 2
            let second_half = self.to % (10_usize.pow(to_digits as u32 / 2)); // 2

            if second_half < first_half {
                first_half - 1
            } else {
                first_half
            }
        } else {
            10_usize.pow((to_digits as u32 - 1) / 2) - 1
        };

        (start..=end)
            .map(|n| format!("{}{}", n, n).parse::<usize>().unwrap())
            .sum()
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_range = s.split('-');
        let from: usize = split_range.next().unwrap().parse().unwrap();
        let to: usize = split_range.next().unwrap().parse().unwrap();

        Ok(Range::new(from, to))
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
