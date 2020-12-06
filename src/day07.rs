use std::fs::File;
use std::io::Read;

use counter::Counter;

type ResultType = ();

fn parse(input: &str) -> ResultType {}

pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day7.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");

    let p = parse(&content);
    println!("Day 07: {} | {}", solve_part1(&p), solve_part2(&p));
}

fn solve_part1(p: &ResultType) -> usize {
    0
}

fn solve_part2(p: &ResultType) -> usize {
    0
}
