use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

const SEL_ROCK: u32 = 1;
const SEL_PAPER: u32 = 2;
const SEL_SCISSORS: u32 = 3;

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Debug)]
struct StrategyTuplePart1 {
    opp_choice: Shape,
    rec_choice: Shape,
}

impl StrategyTuplePart1 {
    fn evaluate(&self) -> u32 {
        match (&self.rec_choice, &self.opp_choice) {
            (Shape::Rock, Shape::Rock) => SEL_ROCK + DRAW,
            (Shape::Rock, Shape::Paper) => SEL_ROCK + LOSE,
            (Shape::Rock, Shape::Scissors) => SEL_ROCK + WIN,
            (Shape::Paper, Shape::Rock) => SEL_PAPER + WIN,
            (Shape::Paper, Shape::Paper) => SEL_PAPER + DRAW,
            (Shape::Paper, Shape::Scissors) => SEL_PAPER + LOSE,
            (Shape::Scissors, Shape::Rock) => SEL_SCISSORS + LOSE,
            (Shape::Scissors, Shape::Paper) => SEL_SCISSORS + WIN,
            (Shape::Scissors, Shape::Scissors) => SEL_SCISSORS + DRAW,
        }
    }
}

#[derive(Debug)]
struct StrategyTuplePart2 {
    opp_choice: Shape,
    rec_choice: Outcome,
}

impl StrategyTuplePart2 {
    fn evaluate(&self) -> u32 {
        match (&self.rec_choice, &self.opp_choice) {
            (Outcome::Win, Shape::Rock) => WIN + SEL_PAPER,
            (Outcome::Win, Shape::Paper) => WIN + SEL_SCISSORS,
            (Outcome::Win, Shape::Scissors) => WIN + SEL_ROCK,
            (Outcome::Draw, Shape::Rock) => DRAW + SEL_ROCK,
            (Outcome::Draw, Shape::Paper) => DRAW + SEL_PAPER,
            (Outcome::Draw, Shape::Scissors) => DRAW + SEL_SCISSORS,
            (Outcome::Lose, Shape::Rock) => LOSE + SEL_SCISSORS,
            (Outcome::Lose, Shape::Paper) => LOSE + SEL_ROCK,
            (Outcome::Lose, Shape::Scissors) => LOSE + SEL_PAPER,
        }
    }
}

fn read_strategy_part1() -> Result<Vec<StrategyTuplePart1>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let tuples: Vec<StrategyTuplePart1> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let opp = match parts[0] {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => panic!(),
            };
            let rec = match parts[1] {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => panic!(),
            };
            StrategyTuplePart1 {
                opp_choice: opp,
                rec_choice: rec,
            }
        })
        .collect();

    Ok(tuples)
}

fn read_strategy_part2() -> Result<Vec<StrategyTuplePart2>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let tuples: Vec<StrategyTuplePart2> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let opp = match parts[0] {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => panic!(),
            };
            let rec = match parts[1] {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => panic!(),
            };
            StrategyTuplePart2 {
                opp_choice: opp,
                rec_choice: rec,
            }
        })
        .collect();

    Ok(tuples)
}

fn part1(strategy: &Vec<StrategyTuplePart1>) -> u32 {
    strategy.iter().map(|tuple| tuple.evaluate()).sum()
}

fn part2(strategy: &Vec<StrategyTuplePart2>) -> u32 {
    strategy.iter().map(|tuple| tuple.evaluate()).sum()
}

fn main() -> Result<()> {
    let start = Instant::now();

    let strategy1 = read_strategy_part1()?;
    let result1 = part1(&strategy1);
    println!("part 1 result: {}", result1);

    let strategy2 = read_strategy_part2()?;
    let result2 = part2(&strategy2);
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
