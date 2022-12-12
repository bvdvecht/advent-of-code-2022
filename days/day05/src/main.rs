use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;

enum Part {
    One,
    Two,
}

enum Input {
    Test,
    Puzzle,
}

#[derive(Debug)]
struct Instruction {
    number: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = text.split_ascii_whitespace().collect();
        Ok(Instruction {
            number: parts[1].parse()?,
            from: parts[3].parse()?,
            to: parts[5].parse()?,
        })
    }
}

#[derive(Debug)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Stack { crates: Vec::new() }
    }

    fn push(&mut self, c: char) -> () {
        self.crates.push(c)
    }

    fn pop(&mut self) -> char {
        self.crates.pop().unwrap()
    }

    fn pop_multiple(&mut self, number: usize) -> Vec<char> {
        let mut crates = Vec::new();
        for _ in 0..number {
            crates.push(self.pop());
        }
        crates.reverse();
        crates
    }

    fn top(&self) -> char {
        *self.crates.last().unwrap()
    }
}

#[derive(Debug)]
struct Cargo {
    stacks: Vec<Stack>,
}

impl Cargo {
    fn move_crates_9000(&mut self, instr: Instruction) -> () {
        for _ in 0..instr.number {
            let crat = self.stacks[instr.from - 1].pop();
            self.stacks[instr.to - 1].push(crat);
        }
    }

    fn move_crates_9001(&mut self, instr: Instruction) -> () {
        let crates = self.stacks[instr.from - 1].pop_multiple(instr.number);
        for c in crates {
            self.stacks[instr.to - 1].push(c);
        }
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.top())
            .collect::<String>()
    }
}

fn solve(part: Part, input: Input) -> Result<String> {
    let file = match input {
        Input::Test => File::open("test.txt")?,
        Input::Puzzle => File::open("input.txt")?,
    };
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let (stack_lines, instr_lines) = lines.split(|line| line.is_empty()).collect_tuple().unwrap();

    // Contruct stacks.
    let mut cargo = Cargo { stacks: Vec::new() };
    let num_stacks = stack_lines[0].len() / 4 + 1;
    for _ in 0..num_stacks {
        cargo.stacks.push(Stack::new());
    }

    for line in stack_lines[0..stack_lines.len() - 1].iter().rev() {
        for i in 0..num_stacks {
            let chars = line.chars().collect::<Vec<char>>();
            let c = chars[i * 4 + 1];
            if !c.is_whitespace() {
                cargo.stacks[i].push(c);
            }
        }
    }

    let instructions: Vec<Instruction> = instr_lines
        .iter()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    for instr in instructions {
        match part {
            Part::One => cargo.move_crates_9000(instr),
            Part::Two => cargo.move_crates_9001(instr),
        };
    }

    Ok(cargo.top_crates())
}

fn main() -> Result<()> {
    let start = Instant::now();

    let test1 = solve(Part::One, Input::Test)?;
    assert_eq!(test1, "CMZ");

    let result1 = solve(Part::One, Input::Puzzle)?;
    println!("part 1 result: {}", result1);

    let test2 = solve(Part::Two, Input::Test)?;
    assert_eq!(test2, "MCD");

    let result2 = solve(Part::Two, Input::Puzzle)?;
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
