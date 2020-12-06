use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
struct Entry<'a> {
    letter: char,
    range: RangeInclusive<u8>,
    password: &'a str,
}

impl<'a> Entry<'a> {
    fn matches(&self) -> bool {
        let size: u8 = self.password.matches(self.letter).count() as u8;

        self.range.contains(&size)
    }

    fn matches_2(&self) -> bool {
        let mut r = self.range.clone();
        let first: usize = r.next().expect("Invalid input") as usize - 1;
        let last: usize = r.last().expect("Invalid input") as usize - 1;
        let chars = self.password.as_bytes();

        let first_char = *chars.get(first).unwrap_or(&0u8) as char;
        let last_char = *chars.get(last).unwrap_or(&0u8) as char;

        (first_char == self.letter) ^ (last_char == self.letter)
    }
}

fn parse(input: &str) -> Vec<Entry> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            let mut elements = l.split_whitespace();
            let mut raw_range = elements.next().expect("Invalid input").split("-");
            let range =
                raw_range.next().expect("Invalid input").parse::<u8>().expect("Invalid input")..=
                    raw_range.next().expect("Invalid input").parse::<u8>().expect("Invalid input");

            let letter: &str = elements.next().expect("Invalid input");
            let password = elements.next().expect("Invalid input");

            Entry {
                letter: letter.chars().next().expect("Invalid input"),
                range,
                password,
            }
        })
        .collect()
}


pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day2.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");

    let inp = parse(&content);
    println!("Day 02: {} | {}", solve_part1(inp.clone()), solve_part2(inp));
}

fn solve_part1(entries: Vec<Entry>) -> i32 {
    entries
        .into_iter()
        .filter(Entry::matches)
        .count() as i32
}

fn solve_part2(entries: Vec<Entry>) -> i32 {
    entries
        .into_iter()
        .filter(Entry::matches_2)
        .count() as i32
}
