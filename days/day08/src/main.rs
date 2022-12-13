use anyhow::Result;
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

#[derive(Debug)]
struct TreePatch {
    trees: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl TreePatch {
    fn new(trees: Vec<Vec<u32>>) -> Self {
        let height = trees.len();
        let width = trees[0].len();
        Self {
            trees: trees,
            height: height,
            width: width,
        }
    }

    fn is_visible_from_left(&self, x: usize, y: usize) -> bool {
        let tree_height = self.trees[y][x];
        self.trees[y][0..x].iter().all(|t| t < &tree_height)
    }

    fn is_visible_from_right(&self, x: usize, y: usize) -> bool {
        let tree_height = self.trees[y][x];
        self.trees[y][x + 1..].iter().all(|t| t < &tree_height)
    }

    fn is_visible_from_top(&self, x: usize, y: usize) -> bool {
        let tree_height = self.trees[y][x];
        (0..y).all(|t| self.trees[t][x] < tree_height)
    }

    fn is_visible_from_bottom(&self, x: usize, y: usize) -> bool {
        let tree_height = self.trees[y][x];
        (y + 1..self.height).all(|t| self.trees[t][x] < tree_height)
    }

    fn viewing_distance_left(&self, x: usize, y: usize) -> usize {
        let tree_height = self.trees[y][x];
        let mut result = 0;
        for i in (0..x).rev() {
            result += 1;
            if self.trees[y][i] >= tree_height {
                break;
            }
        }
        result
    }

    fn viewing_distance_right(&self, x: usize, y: usize) -> usize {
        let tree_height = self.trees[y][x];
        let mut result = 0;
        for i in x + 1..self.width {
            result += 1;
            if self.trees[y][i] >= tree_height {
                break;
            }
        }
        result
    }

    fn viewing_distance_top(&self, x: usize, y: usize) -> usize {
        let tree_height = self.trees[y][x];
        let mut result = 0;
        for i in (0..y).rev() {
            result += 1;
            if self.trees[i][x] >= tree_height {
                break;
            }
        }
        result
    }

    fn viewing_distance_bottom(&self, x: usize, y: usize) -> usize {
        let tree_height = self.trees[y][x];
        let mut result = 0;
        for i in y + 1..self.height {
            result += 1;
            if self.trees[i][x] >= tree_height {
                break;
            }
        }
        result
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        self.is_visible_from_left(x, y)
            || self.is_visible_from_right(x, y)
            || self.is_visible_from_top(x, y)
            || self.is_visible_from_bottom(x, y)
    }

    fn viewing_distance(&self, x: usize, y: usize) -> usize {
        let result = self.viewing_distance_left(x, y)
            * self.viewing_distance_right(x, y)
            * self.viewing_distance_top(x, y)
            * self.viewing_distance_bottom(x, y);
        result
    }
}

fn solve(part: Part, input: Input) -> Result<usize> {
    let file = match input {
        Input::Test => File::open("test.txt")?,
        Input::Puzzle => File::open("input.txt")?,
    };
    let reader = BufReader::new(file);
    let trees: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let patch = TreePatch::new(trees);

    let num_outer = 2 * patch.width + 2 * patch.height - 4;

    let result = match part {
        Part::One => {
            (1..patch.width - 1)
                .map(|x| {
                    (1..patch.height - 1)
                        .map(|y| match patch.is_visible(x, y) {
                            true => 1,
                            false => 0,
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
                + num_outer
        }
        Part::Two => (0..patch.width)
            .map(|x| {
                (0..patch.height)
                    .map(|y| patch.viewing_distance(x, y))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap(),
    };

    Ok(result)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let test1 = solve(Part::One, Input::Test)?;
    assert_eq!(test1, 21);

    let result1 = solve(Part::One, Input::Puzzle)?;
    println!("part 1 result: {}", result1);

    let test2 = solve(Part::Two, Input::Test)?;
    assert_eq!(test2, 8);

    let result2 = solve(Part::Two, Input::Puzzle)?;
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
