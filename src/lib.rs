extern crate itertools;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;


pub fn parse_input_to_numbers(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x|
            x.parse::<i32>().unwrap())
        .collect()
}
