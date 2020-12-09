use itertools::Itertools;

use std::fs::File;
use std::io::Read;
use std::mem;
use std::num::ParseIntError;
use std::slice::Windows;

type ResultType = Vec<i64>;

fn parse(input: &str) -> ResultType {
    input
        .lines()
        .map(|x|
            x.parse::<i64>().expect("Invalid number"))
        .collect()
}

#[derive(Debug)]
struct WindowCheck {
    max: i64,
    min: i64,
    last: i64,
    sums: Vec<i64>,
}

impl<'a> From<&'a [i64]> for WindowCheck {
    fn from(i: &'a [i64]) -> Self {
        let elements = &i[..i.len() - 1];
        let sorted: Vec<&i64> = elements.clone().into_iter().sorted().collect();
        let sums = elements
            .into_iter()
            .cartesian_product(elements)
            .map(|(x, y)| x + y)
            .collect::<Vec<i64>>();

        WindowCheck {
            max: **sorted.get(sorted.len() - 2).expect("..."),
            min: **sorted.get(0).expect("..."),
            last: *i.last().expect("..."),
            sums,
        }
    }
}

impl WindowCheck {
    fn err(&self) -> bool {
        !self.sums.contains(&self.last)
    }
}

pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day9.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");

    let p = parse(&content);

    println!("Day 08: {:?} | {}", solve_part1(p.clone()), solve_part2(p));
}

fn solve_part1(p: ResultType) -> i64 {
    p
        .windows(26)
        .map(WindowCheck::from)
        .find(|w| w.err())
        .expect("no err")
        .last
}

fn solve_part2(p: ResultType) -> i64 {
    let r = solve_part1(p.clone());

    let x = (2..p.len() - 3)
        .filter_map(|k|
            p
                .windows(k)
                .filter_map(|w: &[i64]|
                    if w
                        .into_iter()
                        .fold(0, |a, x| a + x) == r {
                        Some(w)
                    } else {
                        None
                    })
                .next()
        )
        .next()
        .expect("...");

    let mut y = x.to_vec();
    y.sort();
    y[0] + y[y.len() - 1]
}
