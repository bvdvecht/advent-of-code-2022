use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

enum Part {
    One,
    Two,
}

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.min >= other.min && self.min <= other.max)
            || (self.max >= other.min && self.max <= other.max)
            || self.contains(other)
            || other.contains(self)
    }
}

fn solve(part: Part) -> Result<usize> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let contain_count = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (range0, range1) = line
                .split(",")
                .map(|range_str| {
                    let (min, max) = range_str
                        .split("-")
                        .map(|number_str| number_str.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Range { min, max }
                })
                .collect_tuple()
                .unwrap();
            match part {
                Part::One => range0.contains(&range1) || range1.contains(&range0),
                Part::Two => range0.overlaps(&range1),
            }
        })
        .filter(|b| *b)
        .count();

    Ok(contain_count)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let result1 = solve(Part::One)?;
    println!("part 1 result: {}", result1);

    let result2 = solve(Part::Two)?;
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
