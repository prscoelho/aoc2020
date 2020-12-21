#![feature(hash_drain_filter)]
#![feature(drain_filter)]
#![feature(str_split_once)]
#![allow(dead_code)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

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
    println!("Part 1: {}", day21::part1(&contents));
    println!("Part 2: {}", day21::part2(&contents));

    Ok(())
}
