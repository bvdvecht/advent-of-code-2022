use anyhow::Result;
use core::panic;
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

struct SingleFile {
    name: String,
    size: usize,
}

impl SingleFile {
    fn new(name: &str, size: usize) -> Self {
        SingleFile {
            name: String::from(name),
            size: size,
        }
    }
}
struct Directory<'a> {
    name: String,
    subdirs: Vec<Directory<'a>>,
    files: Vec<SingleFile>,
    parent: Option<&'a Directory<'a>>,
}

impl<'a> Directory<'a> {
    fn new(name: &str, parent: Option<&'a Directory>) -> Self {
        Directory {
            name: String::from(name),
            subdirs: Vec::new(),
            files: Vec::new(),
            parent: parent,
        }
    }

    fn get_dir(&mut self, name: &str) -> &mut Directory {
        for dir in &mut self.subdirs {
            if dir.name == name {
                return dir;
            }
        }
        panic!()
    }

    fn size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>()
            + self.subdirs.iter().map(|dir| dir.size()).sum::<usize>()
    }
}

enum Command {
    Cd,
    Ls,
}

impl Command {
    fn parse(text: &str) -> Self {
        match text {
            "cd" => Self::Cd,
            "ls" => Self::Ls,
            _ => panic!(),
        }
    }
}

enum Argument {
    Empty,
    Name(String),
    Root,
    Parent,
}

impl Argument {
    fn parse(text: &str) -> Self {
        match text {
            "/" => Self::Root,
            ".." => Self::Parent,
            s => Self::Name(String::from(s)),
        }
    }
}

struct TerminalInput {
    cmd: Command,
    arg: Argument,
}

enum OutputType {
    Dir,
    Size(usize),
}

struct TerminalOutput {
    typ: OutputType,
    name: String,
}

enum TerminalLine {
    Input(TerminalInput),
    Output(TerminalOutput),
}

fn parse_terminal(reader: BufReader<File>) -> Vec<TerminalLine> {
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            match parts[0] {
                "$" => match parts.len() {
                    2 => TerminalLine::Input(TerminalInput {
                        cmd: Command::parse(parts[1]),
                        arg: Argument::Empty,
                    }),
                    3 => TerminalLine::Input(TerminalInput {
                        cmd: Command::parse(parts[1]),
                        arg: Argument::parse(parts[2]),
                    }),
                    _ => panic!(),
                },
                _ => match parts[0].parse::<usize>() {
                    Ok(size) => TerminalLine::Output(TerminalOutput {
                        typ: OutputType::Size(size),
                        name: String::from(parts[1]),
                    }),
                    Err(_) => TerminalLine::Output(TerminalOutput {
                        typ: OutputType::Dir,
                        name: String::from(parts[1]),
                    }),
                },
            }
        })
        .collect()
}

fn solve(part: Part, input: Input) -> Result<usize> {
    let file = match input {
        Input::Test => File::open("test.txt")?,
        Input::Puzzle => File::open("input.txt")?,
    };
    let reader = BufReader::new(file);
    let lines: Vec<TerminalLine> = parse_terminal(reader);

    let mut root = Directory::new("root", None);

    let mut current_dir = &mut root;
    // let mut current_parent: Option<&Directory> = None;

    for line in lines {
        match line {
            TerminalLine::Input(input) => match input.cmd {
                Command::Cd => match input.arg {
                    Argument::Root => current_dir = &mut root,
                    Argument::Parent => current_dir = unsafe { &mut *current_dir.parent.unwrap() },
                    Argument::Name(s) => current_dir = current_dir.get_dir(&s),
                    _ => panic!(),
                },
                Command::Ls => (),
            },
            TerminalLine::Output(output) => match output.typ {
                OutputType::Dir => current_dir
                    .subdirs
                    .push(Directory::new(&output.name, Some(current_dir))),
                OutputType::Size(size) => {
                    current_dir.files.push(SingleFile::new(&output.name, size))
                }
            },
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
