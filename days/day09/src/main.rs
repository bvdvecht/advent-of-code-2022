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

enum CatchUpMovement {
    UpLeft,
    Up,
    UpRight,
    Left,
    NoMovement,
    Right,
    DownLeft,
    Down,
    DownRight,
}

#[derive(Debug, Clone)]
struct Knot {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    knots: Vec<Knot>,
    visited: Vec<Vec<bool>>,
}

impl Grid {
    fn get_movement(&self, knot0: usize, knot1: usize) -> CatchUpMovement {
        let delta_x = self.knots[knot0].x as i32 - self.knots[knot1].x as i32;
        let delta_y = self.knots[knot0].y as i32 - self.knots[knot1].y as i32;

        match (delta_x, delta_y) {
            (-2, 2) | (-2, 1) | (-1, 2) => CatchUpMovement::UpLeft,
            (0, 2) => CatchUpMovement::Up,
            (2, 2) | (2, 1) | (1, 2) => CatchUpMovement::UpRight,
            (-2, 0) => CatchUpMovement::Left,
            (2, 0) => CatchUpMovement::Right,
            (-2, -2) | (-2, -1) | (-1, -2) => CatchUpMovement::DownLeft,
            (0, -2) => CatchUpMovement::Down,
            (2, -2) | (2, -1) | (1, -2) => CatchUpMovement::DownRight,
            _ => CatchUpMovement::NoMovement,
        }
    }

    fn update_knot(&mut self, knot: usize, movement: CatchUpMovement) {
        match movement {
            CatchUpMovement::UpLeft => {
                self.knots[knot].x -= 1;
                self.knots[knot].y += 1;
            }
            CatchUpMovement::Up => self.knots[knot].y += 1,
            CatchUpMovement::UpRight => {
                self.knots[knot].x += 1;
                self.knots[knot].y += 1;
            }
            CatchUpMovement::Left => self.knots[knot].x -= 1,
            CatchUpMovement::NoMovement => (),
            CatchUpMovement::Right => self.knots[knot].x += 1,
            CatchUpMovement::DownLeft => {
                self.knots[knot].x -= 1;
                self.knots[knot].y -= 1;
            }
            CatchUpMovement::Down => self.knots[knot].y -= 1,
            CatchUpMovement::DownRight => {
                self.knots[knot].x += 1;
                self.knots[knot].y -= 1;
            }
        }
    }

    fn update_knots(&mut self) {
        for i in 1..self.knots.len() {
            let mov = self.get_movement(i - 1, i);
            self.update_knot(i, mov)
        }
        self.mark_visited();
    }

    fn execute_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::Left(n) => {
                for _ in 0..n {
                    self.knots[0].x -= 1;
                    self.update_knots();
                }
            }
            Instruction::Right(n) => {
                for _ in 0..n {
                    self.knots[0].x += 1;
                    self.update_knots();
                }
            }
            Instruction::Up(n) => {
                for _ in 0..n {
                    self.knots[0].y += 1;
                    self.update_knots();
                }
            }
            Instruction::Down(n) => {
                for _ in 0..n {
                    self.knots[0].y -= 1;
                    self.update_knots();
                }
            }
        }
    }

    fn mark_visited(&mut self) {
        let tail = self.knots.last().unwrap();
        self.visited[tail.y][tail.x] = true;
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

    let head = Knot {
        x: start_x,
        y: start_y,
    };
    let tail = Knot {
        x: start_x,
        y: start_y,
    };

    let knots = match part {
        Part::One => vec![head, tail],
        Part::Two => {
            let mut knots = Vec::new();
            knots.push(head);
            for _ in 0..9 {
                knots.push(tail.clone());
            }
            knots
        }
    };

    let mut grid = Grid { knots, visited };

    for instr in instructions {
        grid.execute_instruction(instr);
    }

    let result = grid.count_visited();

    Ok(result)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let test1 = solve(Part::One, Input::Test)?;
    assert_eq!(test1, 13);

    let result1 = solve(Part::One, Input::Puzzle)?;
    println!("part 1 result: {}", result1);

    let test2 = solve(Part::Two, Input::Test)?;
    assert_eq!(test2, 1);

    let result2 = solve(Part::Two, Input::Puzzle)?;
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
