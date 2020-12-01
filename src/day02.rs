use std::fs::File;
use std::io::Read;

use crate::parse_input_to_numbers;

pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day2.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");
    let inp = parse_input_to_numbers(&content);
    println!("Day 02: {} | {}", solve_part1(&inp), solve_part2(&inp));
}

fn solve_part1(input: &[i32]) -> i32 {
    unimplemented!()
}

fn solve_part2(input: &[i32]) -> i32 {
    unimplemented!()
}
