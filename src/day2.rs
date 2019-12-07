use nom::character::complete::digit1;
use std::error::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

named!(parser<&str, Vec<i32>>, separated_list!(
        char!(','), 
        map_res!(recognize!(digit1), FromStr::from_str
        )));

pub fn run() -> Result<(i32, i32), Box<dyn Error>> {
    let mut input = String::new();
    BufReader::new(File::open("inputs/day-2")?).read_line(&mut input)?;
    let mut parsed_input = parser(&input).unwrap().1;
    Ok((part1(&mut parsed_input.clone()), part2(&mut parsed_input)))
}

fn calc(input: &mut Vec<i32>) -> i32 {
    let mut start = 0;
    loop {
        if start >= input.len() {
            break;
        }
        let opcode = input[start];
        let x = input[start + 1] as usize;
        let y = input[start + 2] as usize;
        let r = input[start + 3] as usize;

        match opcode {
            1 => {
                // add
                input[r] = input[x] + input[y];
            }
            2 => {
                // multiply
                input[r] = input[x] * input[y];
            }
            99 => {
                break;
            }
            _ => {
                panic!("{} is an unknown opcode", opcode);
            }
        }
        start += 4;
    }

    input[0]
}

pub fn part1(input: &mut Vec<i32>) -> i32 {
    input[1] = 12;
    input[2] = 2;
    calc(input)
}

fn part2(input: &mut Vec<i32>) -> i32 {
    for a in 0..=99 {
        for b in 0..=99 {
            let mut copy = input.to_vec();
            copy[1] = a;
            copy[2] = b;
            let result = calc(&mut copy);
            if 19_690_720 == result {
                return 100 * a + b;
            }
        }
    }
    0
}
