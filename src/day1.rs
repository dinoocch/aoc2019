use std::error::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Result<(i64, i64), Box<dyn Error>> {
    Ok(BufReader::new(File::open("inputs/day-1")?)
        .lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .fold((0, 0), |acc, x| (acc.0 + part_one(x), acc.1 + part_two(x))))
}

fn part_one(input: i64) -> i64 {
    std::cmp::max(input / 3 - 2, 0)
}

fn part_two(input: i64) -> i64 {
    let mut final_value = 0;
    let mut current_value = part_one(input);

    while current_value > 0 {
        final_value += current_value;
        current_value = part_one(current_value);
    }
    final_value
}
