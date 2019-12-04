use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = BufReader::new(f);
    let result: i32 = f
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .map(|v| v / 3 - 2)
        .sum();
    println!("{}", result);

    Ok(())
}
