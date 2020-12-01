mod day01;

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
    println!("Part 1: {}", day01::part1(&contents));
    println!("Part 2: {}", day01::part2(&contents));

    Ok(())
}
