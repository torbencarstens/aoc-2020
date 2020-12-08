use std::fs::File;
use std::io::Read;
use std::mem;

type ResultType = Vec<Expression<String>>;

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    NOP,
    ACCUMULATOR,
    JUMP,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        match s {
            "nop" => Instruction::NOP,
            "acc" => Instruction::ACCUMULATOR,
            "jmp" => Instruction::JUMP,
            _ => unimplemented!()
        }
    }
}

#[derive(Clone, Debug)]
struct Expression<T> {
    instruction: Instruction,
    arguments: Vec<T>,
}

impl From<&str> for Expression<String> {
    fn from(s: &str) -> Self {
        let mut iter = s.split_whitespace().into_iter();

        Expression {
            instruction: Instruction::from(iter.next().expect("...")),
            arguments: iter.map(|s| s.to_string()).collect(),
        }
    }
}

fn parse(input: &str) -> ResultType {
    input
        .lines()
        .map(Expression::from)
        .collect()
}

pub fn run() {
    let mut content = String::new();
    File::open("input/2020/day8.txt").expect("File not found").read_to_string(&mut content).expect("Couldn't read from file");

    let p = parse(&content);

    println!("Day 08: {:?} | {}", solve_part1(p.clone()), solve_part2(p));
}

fn solve_part1(p: ResultType) -> (i32, bool) {
    let mut pc: i32 = 0;
    let mut a = 0;
    let mut pcs = vec![];

    let mut iter = p.into_iter();
    loop {
        if pc >= iter.clone().count() as i32 {
            return (a, true);
        }

        if pcs.contains(&pc) {
            return (a, false);
        }

        let mut expr = iter.clone().nth(pc as usize).expect(&*format!("wrong pc {}", pc));
        let mut m = 1;

        match expr.instruction {
            Instruction::NOP => {}
            Instruction::ACCUMULATOR => {
                a += expr.arguments.get(0).unwrap().parse::<i32>().expect("Not a number");
            }
            Instruction::JUMP => {
                m = expr.arguments.get(0).unwrap().parse::<i32>().expect("Not a number");
            }
        };

        pcs.push(pc);
        pc += m;
    }
}

fn swap_nop_jmp(i: Instruction) -> Instruction {
    match i {
        Instruction::NOP => {
            Instruction::JUMP
        }
        Instruction::ACCUMULATOR => panic!("swap_nop_jmp can't have acc as argument"),
        Instruction::JUMP => {
            Instruction::NOP
        }
    }
}

fn solve_part2(p: ResultType) -> i32 {
    for (i, e) in p.clone().into_iter().enumerate() {
        if e.instruction == Instruction::NOP || e.instruction == Instruction::JUMP {
            let mut n = p.clone();
            mem::replace(&mut n[i], Expression {
                instruction: swap_nop_jmp(e.instruction),
                arguments: e.arguments,
            });

            match solve_part1(n) {
                (a, true) => {
                    return a;
                }
                _ => {}
            }
        }
    }

    0
}
