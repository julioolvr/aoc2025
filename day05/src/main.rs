use std::{
    cmp, env,
    fs::File,
    io::{self, BufRead},
    ops::RangeInclusive,
};

fn main() {
    let mut lines = read_lines().unwrap().map(|line| line.unwrap());
    let range_lines = lines.by_ref().take_while(|line| line != "");
    let mut ranges: Vec<RangeInclusive<usize>> = range_lines
        .map(|line| {
            let mut split = line.split('-');
            let from: usize = split.next().unwrap().parse().unwrap();
            let to: usize = split.next().unwrap().parse().unwrap();
            from..=to
        })
        .collect();

    let ingredient_ids = lines.map(|line| line.parse::<usize>().unwrap());

    let part_1 = ingredient_ids
        .filter(|ingredient_id| ranges.iter().any(|r| r.contains(ingredient_id)))
        .count();
    println!("Part 1: {}", part_1);

    ranges.sort_by_key(|range| *range.start());
    let merged_ranges = ranges
        .iter()
        .skip(1)
        .fold(vec![ranges[0].clone()], |mut acc, range| {
            let last_range = acc.last().unwrap();
            if last_range.end() >= range.start() {
                let len = acc.len();
                acc[len - 1] = (*last_range.start())..=(cmp::max(*range.end(), *last_range.end()));
            } else {
                acc.push(range.clone());
            }

            acc
        });

    let part_2: usize = merged_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum();

    println!("Part 2: {}", part_2);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
