use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines: Vec<Vec<String>> = read_lines()
        .unwrap()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .collect();
    let mut operands: Vec<Vec<usize>> = lines[0].iter().map(|_| vec![]).collect();

    for line in &lines[0..lines.len() - 1] {
        for (i, operand) in line.iter().enumerate() {
            operands[i].push(operand.parse::<usize>().unwrap());
        }
    }
    let operations = lines.last().unwrap();

    let mut part_1 = 0;
    for (i, operand_list) in operands.iter().enumerate() {
        part_1 += operand_list
            .iter()
            .copied()
            .reduce(|acc, n| match operations[i].as_str() {
                "+" => acc + n,
                "*" => acc * n,
                foo => panic!("Unexpected operation {}", foo),
            })
            .unwrap();
    }
    println!("Part 1: {}", part_1);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
