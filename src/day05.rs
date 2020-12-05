use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Index, Range, RangeInclusive};
use std::str::Chars;

use itertools::Itertools;

fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|l|
            l
                .replace("R", "1")
                .replace("B", "1")
                .replace("F", "0")
                .replace("L", "0")
        )
        .map(|s| isize::from_str_radix(&s, 2).unwrap())
        .collect()
}

pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day5.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");

    let p = parse(&content);
    println!("Day 05: {} | {}", solve_part1(&p), solve_part2(&p));
}

fn solve_part1(p: &[isize]) -> isize {
    *p
        .into_iter()
        .max()
        .expect("No max")
}

fn solve_part2(p: &[isize]) -> isize {
    p
        .into_iter()
        .sorted()
        .tuple_windows::<(&isize, &isize)>()
        .find(|(x, y)| **x + 2 == **y)
        .expect("Your seat is not on this airplane")
        .1 - 1
}
