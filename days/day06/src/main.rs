use anyhow::Result;
use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

enum Part {
    One,
    Two,
}

enum Input {
    Test,
    Puzzle,
}

fn all_different(chars: &[char], len: usize) -> bool {
    let set: HashSet<char> = HashSet::from_iter(chars.to_owned());
    set.len() == len
}

fn solve(part: Part, input: Input) -> Result<usize> {
    let file = match input {
        Input::Test => File::open("test.txt")?,
        Input::Puzzle => File::open("input.txt")?,
    };
    let reader = BufReader::new(file);
    let chars: Vec<char> = reader
        .lines()
        .map(|line| line.unwrap())
        .next()
        .unwrap()
        .chars()
        .collect();

    let marker_len = match part {
        Part::One => 4,
        Part::Two => 14,
    };

    for i in marker_len..chars.len() {
        if all_different(&chars[i - marker_len..i], marker_len) {
            return Ok(i);
        }
    }

    panic!()
}

fn main() -> Result<()> {
    let start = Instant::now();

    let test1 = solve(Part::One, Input::Test)?;
    assert_eq!(test1, 7);

    let result1 = solve(Part::One, Input::Puzzle)?;
    println!("part 1 result: {}", result1);

    let test2 = solve(Part::Two, Input::Test)?;
    assert_eq!(test2, 19);

    let result2 = solve(Part::Two, Input::Puzzle)?;
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
