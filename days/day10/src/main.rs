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

enum Instruction {
    Noop,
    Addx(i32),
}

struct Screen {
    curr_pixel_x: usize,
    curr_pixel_y: usize,
    lines: Vec<Vec<char>>,
}

impl Screen {
    fn new() -> Self {
        Self {
            curr_pixel_x: 0,
            curr_pixel_y: 0,
            lines: vec![vec!['.'; 40]; 6],
        }
    }

    fn render(&mut self, cycle: usize, sprite_x: usize) {
        let line = &mut self.lines[((cycle - 1) / 40)];
        let pixel_x = (cycle - 1) % 40;
        if pixel_x >= sprite_x - 1 && pixel_x <= sprite_x + 1 {
            line[pixel_x] = '#';
        }
    }

    fn print(&self) {
        println!("");
        for line in &self.lines {
            println!("{}", line.iter().collect::<String>())
        }
    }
}

fn solve(part: Part, input: Input) -> Result<i32> {
    let file = match input {
        Input::Test => File::open("test.txt")?,
        Input::Puzzle => File::open("input.txt")?,
    };
    let reader = BufReader::new(file);
    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            match parts[..] {
                ["noop"] => Instruction::Noop,
                ["addx", n] => Instruction::Addx(n.parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect();

    let mut reg = 1;
    let mut cycle = 1;

    let mut strength: i32 = 0;
    let mut screen = Screen::new();

    for instr in instructions {
        match instr {
            Instruction::Noop => {
                if ((cycle - 20) % 40) == 0 {
                    strength += cycle * reg;
                }
                screen.render(cycle as usize, reg as usize);
                cycle += 1;
            }
            Instruction::Addx(n) => {
                if ((cycle - 20) % 40) == 0 {
                    strength += cycle * reg;
                } else if ((cycle - 20) % 40) == 39 {
                    strength += (cycle + 1) * reg;
                }
                screen.render(cycle as usize, reg as usize);
                screen.render((cycle + 1) as usize, reg as usize);
                cycle += 2;
                reg += n;
            }
        }
    }

    screen.print();

    Ok(strength)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let test1 = solve(Part::One, Input::Test)?;
    assert_eq!(test1, 13140);

    let result1 = solve(Part::One, Input::Puzzle)?;
    println!("part 1 result: {}", result1);

    let test2 = solve(Part::Two, Input::Test)?;
    assert_eq!(test2, 13140);

    let result2 = solve(Part::Two, Input::Puzzle)?;
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
