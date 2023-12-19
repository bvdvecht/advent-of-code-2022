use anyhow::Result;
use itertools::Itertools;

#[macro_use]
extern crate log;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path;
use std::time::Instant;

#[derive(Clone)]
enum Part {
    One,
    Two,
}

enum Input {
    Test,
    Puzzle,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    squares: Vec<Vec<u8>>,
    end_point: Location,
}

impl Grid {
    fn width(&self) -> usize {
        self.squares[0].len()
    }

    fn height(&self) -> usize {
        self.squares.len()
    }

    fn get_height(&self, loc: Location) -> u8 {
        self.squares[loc.y][loc.x]
    }

    fn step_right(&self, loc: Location) -> Option<Location> {
        if loc.x < self.width() - 1 {
            Some(Location {
                x: loc.x + 1,
                y: loc.y,
            })
        } else {
            None
        }
    }
    fn step_left(&self, loc: Location) -> Option<Location> {
        if loc.x > 0 {
            Some(Location {
                x: loc.x - 1,
                y: loc.y,
            })
        } else {
            None
        }
    }
    fn step_up(&self, loc: Location) -> Option<Location> {
        if loc.y > 0 {
            Some(Location {
                x: loc.x,
                y: loc.y - 1,
            })
        } else {
            None
        }
    }
    fn step_down(&self, loc: Location) -> Option<Location> {
        if loc.y < self.height() - 1 {
            Some(Location {
                x: loc.x,
                y: loc.y + 1,
            })
        } else {
            None
        }
    }
}

fn search(
    grid: &Grid,
    current_loc: Location,
    solutions: &mut HashMap<Location, usize>,
    visited_locs: &mut HashSet<Location>,
) -> Option<usize> {
    debug!("searching from {:?}", current_loc);

    if solutions.contains_key(&current_loc) {
        let solution = *solutions.get(&current_loc).unwrap();
        debug!(
            "using already found solution for {:?}: {}",
            &current_loc, solution
        );
        Some(solution)
    } else {
        let mut solution = 1_000_000;

        let current_height = grid.get_height(current_loc);
        debug!("inserting visited loc {:?}", current_loc);
        visited_locs.insert(current_loc);

        let left_blocked = match grid.step_left(current_loc) {
            Some(loc) => visited_locs.contains(&loc),
            _ => true,
        };
        let right_blocked = match grid.step_right(current_loc) {
            Some(loc) => visited_locs.contains(&loc),
            _ => true,
        };
        let up_blocked = match grid.step_up(current_loc) {
            Some(loc) => visited_locs.contains(&loc),
            _ => true,
        };
        let down_blocked = match grid.step_down(current_loc) {
            Some(loc) => visited_locs.contains(&loc),
            _ => true,
        };

        let mut path_blocked = false;

        if left_blocked && right_blocked && up_blocked && down_blocked {
            debug!("path blocked for {:?}, backtracking", current_loc);
            path_blocked = true;
        }

        if path_blocked {
            return None;
        }

        if let Some(loc) = grid.step_left(current_loc) {
            debug!("current: {:?}, trying left", current_loc);
            if visited_locs.contains(&loc) {
                debug!("skipping left since already visited");
            // } else if grid.get_height(loc) <= current_height + 1 {
            } else if grid.get_height(loc) >= current_height - 1 {
                match search(grid, loc, solutions, visited_locs) {
                    Some(sol) => {
                        if 1 + sol < solution {
                            debug!(
                                "updating solution for {:?} from left: {}",
                                current_loc,
                                1 + sol
                            );
                            solution = 1 + sol;
                        }
                    }
                    _ => (),
                }
            }
        }
        if let Some(loc) = grid.step_right(current_loc) {
            debug!("current: {:?}, trying right", current_loc);
            if visited_locs.contains(&loc) {
                debug!("skipping right since already visited");
            // } else if grid.get_height(loc) <= current_height + 1 {
            } else if grid.get_height(loc) >= current_height - 1 {
                match search(grid, loc, solutions, visited_locs) {
                    Some(sol) => {
                        if 1 + sol < solution {
                            debug!(
                                "updating solution for {:?} from left: {}",
                                current_loc,
                                1 + sol
                            );
                            solution = 1 + sol;
                        }
                    }
                    _ => (),
                }
            }
        }
        if let Some(loc) = grid.step_up(current_loc) {
            debug!("current: {:?}, trying up", current_loc);
            if visited_locs.contains(&loc) {
                debug!("skipping up since already visited");
            // } else if grid.get_height(loc) <= current_height + 1 {
            } else if grid.get_height(loc) >= current_height - 1 {
                match search(grid, loc, solutions, visited_locs) {
                    Some(sol) => {
                        if 1 + sol < solution {
                            debug!(
                                "updating solution for {:?} from left: {}",
                                current_loc,
                                1 + sol
                            );
                            solution = 1 + sol;
                        }
                    }
                    _ => (),
                }
            }
        }
        if let Some(loc) = grid.step_down(current_loc) {
            debug!("current: {:?}, trying down", current_loc);
            if visited_locs.contains(&loc) {
                debug!("skipping down since already visited");
            // } else if grid.get_height(loc) <= current_height + 1 {
            } else if grid.get_height(loc) >= current_height - 1 {
                match search(grid, loc, solutions, visited_locs) {
                    Some(sol) => {
                        if 1 + sol < solution {
                            debug!(
                                "updating solution for {:?} from left: {}",
                                current_loc,
                                1 + sol
                            );
                            solution = 1 + sol;
                        }
                    }
                    _ => (),
                }
            }
        }

        debug!("removing visited loc {:?}", current_loc);
        visited_locs.remove(&current_loc);

        if solution == 1_000_000 {
            return None;
        }

        solutions.insert(current_loc, solution);
        debug!("solution for {:?} = {}", current_loc, solution);
        Some(solution)
    }
}

fn solve(part: Part, input: Input) -> Result<usize> {
    let file = match input {
        Input::Test => File::open("test.txt")?,
        Input::Puzzle => File::open("input.txt")?,
    };
    let reader = BufReader::new(file);

    let squares: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => 27,
                    'E' => 28,
                    _ => c as u8 - 96,
                })
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut grid = Grid {
        squares,
        end_point: Location { x: 0, y: 0 },
    };
    // println!("{:?}", grid);

    let mut start = Location { x: 0, y: 0 };
    let mut end = Location { x: 0, y: 0 };

    for i in 0..grid.width() {
        for j in 0..grid.height() {
            if grid.get_height(Location { x: i, y: j }) == 27 {
                start = Location { x: i, y: j };
            } else if grid.get_height(Location { x: i, y: j }) == 28 {
                end = Location { x: i, y: j };
            }
        }
    }

    grid.squares[start.y][start.x] = 1;
    grid.squares[end.y][end.x] = 26;
    grid.end_point = end;

    println!("start: {:?}", start);
    println!("end: {:?}", end);

    let mut solutions = HashMap::new();
    // solutions.insert(grid.end_point, 0);
    solutions.insert(start, 0);
    let mut visited_locs = HashSet::new();
    // let result = search(&grid, start, &mut solutions, &mut visited_locs);
    let result = search(&grid, end, &mut solutions, &mut visited_locs);

    Ok(result.unwrap())
}

fn main() -> Result<()> {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("day12.log").unwrap(),
    );

    info!("hello");

    let start = Instant::now();

    // let test1 = solve(Part::One, Input::Test)?;
    // assert_eq!(test1, 31);

    let result1 = solve(Part::One, Input::Puzzle)?;
    println!("part 1 result: {}", result1);

    // let test2 = solve(Part::Two, Input::Test)?;
    // assert_eq!(test2, 2713310158);

    // let result2 = solve(Part::Two, Input::Puzzle)?;
    // println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
