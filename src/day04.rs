use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug)]
struct Height {
    size: i32,
    unit: String,
}

impl Height {
    fn validate(&self) -> bool {
        match self.unit.as_str() {
            "cm" => {
                (150..=193).contains(&self.size)
            }
            "in" => {
                (59..=76).contains(&self.size)
            }
            _ => false
        }
    }
}

impl From<&str> for Height {
    fn from(s: &str) -> Self {
        let mut chars = s
            .chars()
            .into_iter();
        let size = chars
            .by_ref()
            .take_while(|c| c.is_numeric())
            .collect::<String>();
        let unit = s.chars().skip(size.len()).collect();

        Height {
            size: size.parse::<i32>().expect("Invalid height string"),
            unit,
        }
    }
}

#[derive(Clone, Debug)]
struct Passport {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: Height,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<i32>,
}

impl Passport {
    fn validate_hair_color(c: &str) -> bool {
        c.len() == 7
            && c.chars().nth(0).unwrap() == '#'
            && c.chars().skip(1).filter(|c|
            (48..=57)
                .chain(97..=102)
                .map(char::from)
                .any(|_c| &_c == c)
        ).count() == 6
    }

    fn validate(&self) -> bool {
        (1920..=2002).contains(&self.byr)
            && (2010..=2020).contains(&self.iyr)
            && (2020..=2030).contains(&self.eyr)
            && self.hgt.validate()
            && Passport::validate_hair_color(&self.hcl)
            && vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_str())
            && self.pid.chars().filter(|c| c.is_numeric()).count() == 9
    }

    fn parse(s: String) -> Option<Self> {
        let mut map: HashMap<&str, &str> = HashMap::new();
        s
            .split_whitespace()
            .into_iter()
            .for_each(|c| {
                let mut s = c.split(":");
                map.insert(s.next().unwrap(), s.next().unwrap());
            });

        Some(Passport {
            byr: map.get("byr")?.parse::<i32>().ok()?,
            iyr: map.get("iyr")?.parse::<i32>().ok()?,
            eyr: map.get("eyr")?.parse::<i32>().ok()?,
            hgt: Height::from(*map.get("hgt")?),
            hcl: map.get("hcl")?.to_string(),
            ecl: map.get("ecl")?.to_string(),
            pid: map.get("pid")?.to_string(),
            cid: match map.get("cid") {
                None => {
                    Some(0)
                }
                Some(v) => {
                    v.parse::<i32>().ok()
                }
            },
        })
    }
}

fn parse(input: &str) -> Vec<Option<Passport>> {
    input
        .split("\n\n")
        .map(|l| l.replace("\n", " "))
        .map(Passport::parse)
        .collect()
}

pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day4.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");

    let p = parse(&content);
    println!("Day 04: {} | {}", solve_part1(p.clone()), solve_part2(p));
}

fn solve_part1(p: Vec<Option<Passport>>) -> i64 {
    p
        .into_iter()
        .filter(|p| p.is_some())
        .count() as i64
}

fn solve_part2(p: Vec<Option<Passport>>) -> i64 {
    p
        .into_iter()
        .filter_map(|p| p)
        .filter(Passport::validate)
        .count() as i64
}
