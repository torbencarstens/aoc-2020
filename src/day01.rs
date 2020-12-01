use std::fs::File;
use std::io::Read;

use itertools::Itertools;

use crate::parse_input_to_numbers;

pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day1.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");
    let inp = parse_input_to_numbers(&content);
    println!("Day 01: {} | {}", solve_part1(&inp), solve_part2(&inp));
}

fn solve_part1(input: &[i32]) -> i32 {
    input
        .into_iter()
        .cartesian_product(input)
        .filter(|(x, y)|
            **x + **y == 2020
        )
        .map(|(x, y)| x * y)
        .next()
        .unwrap()
}

fn solve_part2(input: &[i32]) -> i32 {
    input
        .into_iter()
        .cartesian_product(input)
        .cartesian_product(input)
        .filter(|((x, y), z)|
            **x + **y + **z == 2020
        )
        .map(|((x, y), z)| x * y * z)
        .next()
        .unwrap()
}
