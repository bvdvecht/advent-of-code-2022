use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn read_calories() -> Result<Vec<Vec<u32>>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let calories: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|line_group| {
            line_group
                .iter()
                .map(|line| line.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    Ok(calories)
}

fn part1(calories: &Vec<Vec<u32>>) -> u32 {
    let sums: Vec<u32> = calories.iter().map(|group| group.iter().sum()).collect();
    *sums.iter().max().unwrap()
}

fn part2(calories: &Vec<Vec<u32>>) -> u32 {
    let mut sums: Vec<u32> = calories.iter().map(|group| group.iter().sum()).collect();
    sums.sort();
    sums.reverse();
    assert!(sums.len() >= 3);
    sums[0..3].iter().sum()
}

fn main() -> Result<()> {
    let start = Instant::now();

    let calories = read_calories()?;

    let result1 = part1(&calories);
    println!("part 1 result: {}", result1);

    let result2 = part2(&calories);
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
