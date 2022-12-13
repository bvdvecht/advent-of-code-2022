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

enum Input {
    Test,
    Puzzle,
}

// Position of head w.r.t. tail
enum HeadTailConfig {
    TopLeft,
    Top,
    TopRight,
    Left,
    Overlap,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
    head_x: usize,
    head_y: usize,
    tail_x: usize,
    tail_y: usize,
    visited: Vec<Vec<bool>>,
}

impl Grid {
    fn current_config(&self) -> HeadTailConfig {
        let delta_x = self.head_x as i32 - self.tail_x as i32;
        let delta_y = self.head_y as i32 - self.tail_y as i32;
        match (delta_x, delta_y) {
            (-1, 1) => HeadTailConfig::TopLeft,
            (0, 1) => HeadTailConfig::Top,
            (1, 1) => HeadTailConfig::TopRight,
            (-1, 0) => HeadTailConfig::Left,
            (0, 0) => HeadTailConfig::Overlap,
            (1, 0) => HeadTailConfig::Right,
            (-1, -1) => HeadTailConfig::BottomLeft,
            (0, -1) => HeadTailConfig::Bottom,
            (1, -1) => HeadTailConfig::BottomRight,
            _ => panic!(),
        }
    }

    fn single_step_left(&mut self) {
        let config = self.current_config();
        self.head_x -= 1;
        match config {
            HeadTailConfig::TopLeft => {
                self.tail_x -= 1;
                self.tail_y += 1;
            }
            HeadTailConfig::Left => self.tail_x -= 1,
            HeadTailConfig::BottomLeft => {
                self.tail_x -= 1;
                self.tail_y -= 1;
            }
            _ => (),
        }
    }
    fn single_step_right(&mut self) {
        let config = self.current_config();
        self.head_x += 1;
        match config {
            HeadTailConfig::TopRight => {
                self.tail_x += 1;
                self.tail_y += 1;
            }
            HeadTailConfig::Right => self.tail_x += 1,
            HeadTailConfig::BottomRight => {
                self.tail_x += 1;
                self.tail_y -= 1;
            }
            _ => (),
        }
    }
    fn single_step_up(&mut self) {
        let config = self.current_config();
        self.head_y += 1;
        match config {
            HeadTailConfig::TopLeft => {
                self.tail_x -= 1;
                self.tail_y += 1;
            }
            HeadTailConfig::Top => self.tail_y += 1,
            HeadTailConfig::TopRight => {
                self.tail_x += 1;
                self.tail_y += 1;
            }
            _ => (),
        }
    }
    fn single_step_down(&mut self) {
        let config = self.current_config();
        self.head_y -= 1;
        match config {
            HeadTailConfig::BottomLeft => {
                self.tail_x -= 1;
                self.tail_y -= 1;
            }
            HeadTailConfig::Bottom => self.tail_y -= 1,
            HeadTailConfig::BottomRight => {
                self.tail_x += 1;
                self.tail_y -= 1;
            }
            _ => (),
        }
    }

    fn execute_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::Left(n) => {
                for _ in 0..n {
                    self.single_step_left();
                    self.mark_visited();
                }
            }
            Instruction::Right(n) => {
                for _ in 0..n {
                    self.single_step_right();
                    self.mark_visited();
                }
            }
            Instruction::Up(n) => {
                for _ in 0..n {
                    self.single_step_up();
                    self.mark_visited();
                }
            }
            Instruction::Down(n) => {
                for _ in 0..n {
                    self.single_step_down();
                    self.mark_visited();
                }
            }
        }
    }

    fn mark_visited(&mut self) {
        self.visited[self.tail_y][self.tail_x] = true;
    }

    fn count_visited(&self) -> usize {
        self.visited
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }
}

enum Instruction {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

fn solve(part: Part, input: Input) -> Result<usize> {
    let file = match input {
        Input::Test => File::open("test.txt")?,
        Input::Puzzle => File::open("input.txt")?,
    };
    let reader = BufReader::new(file);
    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (dir, dist) = line.split_ascii_whitespace().collect_tuple().unwrap();
            match (dir, dist) {
                ("L", n) => Instruction::Left(n.parse().unwrap()),
                ("R", n) => Instruction::Right(n.parse().unwrap()),
                ("U", n) => Instruction::Up(n.parse().unwrap()),
                ("D", n) => Instruction::Down(n.parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect();

    let max_dist_left: usize = instructions
        .iter()
        .map(|instr| match instr {
            Instruction::Left(n) => *n,
            _ => 0,
        })
        .sum();
    let max_dist_right: usize = instructions
        .iter()
        .map(|instr| match instr {
            Instruction::Right(n) => *n,
            _ => 0,
        })
        .sum();
    let max_dist_up: usize = instructions
        .iter()
        .map(|instr| match instr {
            Instruction::Up(n) => *n,
            _ => 0,
        })
        .sum();
    let max_dist_down: usize = instructions
        .iter()
        .map(|instr| match instr {
            Instruction::Down(n) => *n,
            _ => 0,
        })
        .sum();

    let width = max_dist_left + max_dist_right + 1;
    let height = max_dist_up + max_dist_down + 1;
    let start_x = max_dist_left;
    let start_y = max_dist_down;

    let visited = vec![vec![false; width]; height];

    let mut grid = Grid {
        width: width,
        height: height,
        start_x: start_x,
        start_y: start_y,
        head_x: start_x,
        head_y: start_y,
        tail_x: start_x,
        tail_y: start_y,
        visited: visited,
    };

    for instr in instructions {
        grid.execute_instruction(instr);
    }

    let result = match part {
        Part::One => grid.count_visited(),
        Part::Two => 42,
    };

    Ok(result)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let test1 = solve(Part::One, Input::Test)?;
    assert_eq!(test1, 13);

    let result1 = solve(Part::One, Input::Puzzle)?;
    println!("part 1 result: {}", result1);

    let test2 = solve(Part::Two, Input::Test)?;
    assert_eq!(test2, 8);

    let result2 = solve(Part::Two, Input::Puzzle)?;
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
