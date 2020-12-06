use std::fs::File;
use std::io::Read;

use counter::Counter;

fn parse(input: &str) -> Vec<(usize, usize, Counter<char>)> {
    input
        .split("\n\n")
        .map(|s|
            (s.lines().count(), s.replace("\n", "")))
        .map(|(pc, s)| {
            let chars = s.chars();
            (chars.clone().count(), pc, chars.collect::<Counter<_>>())
        })
        .collect()
}

pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day6.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");

    let p = parse(&content);
    println!("Day 06: {} | {}", solve_part1(&p), solve_part2(&p));
}

fn solve_part1(p: &Vec<(usize, usize, Counter<char>)>) -> usize {
    p
        .into_iter()
        .map(|(_, _, c)|
            c
                .values()
                .count())
        .sum()
}

fn solve_part2(p: &Vec<(usize, usize, Counter<char>)>) -> usize {
    p
        .into_iter()
        .map(|(_, pc, c)|
            c
                .values()
                .filter(|v| {
                    *v == pc
                })
                .count()
        )
        .sum()
}
