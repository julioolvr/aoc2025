use std::{
    env,
    fs::File,
    io::{self, BufRead},
    ops::RangeInclusive,
};

fn main() {
    let mut lines = read_lines().unwrap().map(|line| line.unwrap());
    let range_lines = lines.by_ref().take_while(|line| line != "");
    let ranges: Vec<RangeInclusive<usize>> = range_lines
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
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
