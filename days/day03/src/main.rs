use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug)]
struct Rucksack {
    comp1: HashSet<char>,
    comp2: HashSet<char>,
}

impl Rucksack {
    fn common_item(&self) -> char {
        let mut common_chars = self.comp1.intersection(&self.comp2);
        *common_chars.next().unwrap()
    }

    fn all_items(&self) -> HashSet<char> {
        self.comp1.union(&self.comp2).map(|c| *c).collect()
    }
}

fn priority(letter: char) -> u8 {
    if letter.is_uppercase() {
        letter as u8 - 38
    } else {
        letter as u8 - 96
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

fn read_input_part2() -> Result<Vec<Vec<Rucksack>>> {
    let rucksacks: Vec<Rucksack> = read_input_part1()?;

    let chunked: Vec<Vec<Rucksack>> = rucksacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|c| c.collect())
        .collect();

    Ok(chunked)
}

fn part1(rucksacks: &Vec<Rucksack>) -> u32 {
    rucksacks
        .iter()
        .map(|r| priority(r.common_item()) as u32)
        .sum()
}

fn part2(rucksacks: &Vec<Vec<Rucksack>>) -> u32 {
    rucksacks
        .iter()
        .map(|group| {
            let all_items: Vec<HashSet<char>> =
                group.iter().map(|rucksack| rucksack.all_items()).collect();

            let mut iterator = all_items.into_iter();
            let first_set = iterator.next().unwrap();

            let common_letters = iterator.fold(first_set, |common, items| {
                common.intersection(&items).map(|c| *c).collect()
            });
            let common_letter: char = *common_letters.iter().next().unwrap();
            priority(common_letter) as u32
        })
        .sum()
}

fn main() -> Result<()> {
    let start = Instant::now();

    let input1 = read_input_part1()?;
    let result1 = part1(&input1);
    println!("part 1 result: {}", result1);

    let input2 = read_input_part2()?;
    let result2 = part2(&input2);
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
