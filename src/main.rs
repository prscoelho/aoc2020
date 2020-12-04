#![allow(dead_code)]
mod day01;
mod day02;
mod day03;
mod day04;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Input file missing");
        return Ok(());
    }

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("Part 1: {}", day03::part1(&contents));
    println!("Part 2: {}", day03::part2(&contents));

    Ok(())
}
