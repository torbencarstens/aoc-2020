use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Index, Range, RangeInclusive};

#[derive(Clone, Copy, Debug, PartialEq)]
enum FieldType {
    Tree,
    Empty,
}

impl From<char> for FieldType {
    fn from(c: char) -> Self {
        match c {
            '#' => FieldType::Tree,
            '.' => FieldType::Empty,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug)]
struct Row {
    elements: Vec<FieldType>
}

impl Row {
    fn get_column_mod(&self, x: usize) -> FieldType {
        *self
            .elements
            .get(x % self.elements.len())
            .expect("...")
    }
}

impl From<&str> for Row {
    fn from(l: &str) -> Self {
        Row {
            elements: l
                .chars()
                .map(FieldType::from)
                .collect()
        }
    }
}

struct Map {
    fields: Vec<Row>
}

impl Map {
    fn get_row(&self, y: usize) -> Option<&Row> {
        self
            .fields
            .get(y)
    }

    fn get(&self, x: i64, y: i64) -> Option<FieldType> {
        Some(self
            .get_row(y as usize)?
            .get_column_mod(x as usize))
    }
}

struct Step {
    x: i64,
    y: i64,
}

struct Position {
    x: i64,
    y: i64,
}

impl Add<&Step> for &Position {
    type Output = Position;

    fn add(self, rhs: &Step) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Walker<'a> {
    current: Position,
    step: Step,
    map: &'a Map,
}

impl<'a> Walker<'a> {
    fn new(step: Step, map: &'a Map) -> Self {
        Walker {
            current: Position { x: 0, y: 0 },
            step,
            map,
        }
    }
}

impl<'a> Iterator for Walker<'a> {
    type Item = Option<FieldType>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.map.get(self.current.x, self.current.y);

        let a = 1;
        match result {
            None => {
                None
            }
            Some(s) => {
                self.current = &self.current + &self.step;
                Some(Some(s))
            }
        }
    }
}

fn parse(input: &str) -> Map {
    Map {
        fields: input
            .lines()
            .into_iter()
            .map(Row::from)
            .collect()
    }
}

pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day3.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");

    let map = parse(&content);
    let w1 = Walker::new(Step { x: 1, y: 2 }, &map);

    println!("Day 03: {} | {}", solve_part1(w1), solve_part2(map));
}

fn solve_part1(w: Walker) -> i64 {
    let n = w.map.fields.len();

    w
        .into_iter()
        .take(n)
        .filter(|f| match f {
            None => {
                false
            }
            Some(ft) => {
                ft == &FieldType::Tree
            }
        })
        .count() as i64
}

fn solve_part2(map: Map) -> i64 {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(x, y)|
            Step { x, y }
        )
        .map(|step|
            Walker::new(step, &map))
        .map(solve_part1)
        .fold(1, |a, x| a * x)
}
