use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

struct Rucksack {
    comp1: HashSet<char>,
    comp2: HashSet<char>,
}

impl Rucksack {
    fn common_item(&self) -> char {
        let mut common_chars = self.comp1.intersection(&self.comp2);
        *common_chars.next().unwrap()
    }

    fn priority(&self) -> u8 {
        let letter = self.common_item();
        if letter.is_uppercase() {
            letter as u8 - 38
        } else {
            letter as u8 - 96
        }
    }
}

fn read_input_part1() -> Result<Vec<Rucksack>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let rucksacks: Vec<Rucksack> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|chars| {
            let (left, right) = chars.split_at(chars.len() / 2);
            Rucksack {
                comp1: HashSet::from_iter(left.to_owned()),
                comp2: HashSet::from_iter(right.to_owned()),
            }
        })
        .collect();

    Ok(rucksacks)
}

fn part1(rucksacks: &Vec<Rucksack>) -> u32 {
    rucksacks.iter().map(|r| r.priority() as u32).sum()
}

fn main() -> Result<()> {
    let start = Instant::now();

    let input1 = read_input_part1()?;
    let result1 = part1(&input1);
    println!("part 1 result: {}", result1);

    // let strategy2 = read_input_part2()?;
    // let result2 = part2(&strategy2);
    // println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
