use anyhow::Result;
use core::panic;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::rc::Rc;
use std::rc::Weak;
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

#[derive(Debug)]
struct Directory {
    name: String,
    subdirs: Vec<Rc<RefCell<Directory>>>,
    files: Vec<SingleFile>,
    parent: Option<Weak<RefCell<Directory>>>,
}

impl Directory {
    fn new(name: &str, parent: Option<Weak<RefCell<Directory>>>) -> Self {
        Directory {
            name: String::from(name),
            subdirs: Vec::new(),
            files: Vec::new(),
            parent: parent,
        }
    }

    fn get_dir(&self, name: &str) -> Rc<RefCell<Directory>> {
        for dir in &self.subdirs {
            if dir.borrow_mut().name == name {
                return dir.clone();
            }
        }
        panic!()
    }

    fn size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>()
            + self
                .subdirs
                .iter()
                .map(|dir| dir.borrow_mut().size())
                .sum::<usize>()
    }

    fn sum_subdirs_with_size(&self, at_most: usize) -> usize {
        let mut result = 0;
        if self.size() <= at_most {
            result += self.size();
        }
        for dir in &self.subdirs {
            let borrowed = dir.borrow_mut();
            result += borrowed.sum_subdirs_with_size(at_most);
        }

        result
    }

    fn smallest_subdir_with_size(&self, at_least: usize) -> usize {
        let mut result = usize::MAX;

        let self_size = self.size();
        if self_size >= at_least && self_size < result {
            result = self_size;
        }
        for dir in &self.subdirs {
            let borrowed = dir.borrow_mut();
            let sub_result = borrowed.smallest_subdir_with_size(at_least);
            if sub_result < result {
                result = sub_result;
            }
        }
        result
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

    let root = Directory::new("root", None);
    let root = Rc::new(RefCell::new(root));

    let mut current_dir: Rc<RefCell<Directory>> = root.clone();

    for line in lines {
        match line {
            TerminalLine::Input(input) => match input.cmd {
                Command::Cd => match input.arg {
                    Argument::Root => current_dir = root.clone(),
                    Argument::Parent => {
                        let idk = current_dir
                            .borrow_mut()
                            .parent
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap();
                        current_dir = idk
                    }
                    Argument::Name(s) => {
                        let idk = current_dir.borrow_mut().get_dir(&s).clone();
                        current_dir = idk
                    }
                    _ => panic!(),
                },
                Command::Ls => (),
            },
            TerminalLine::Output(output) => match output.typ {
                OutputType::Dir => {
                    let mut idk = current_dir.borrow_mut();
                    let weak = Rc::downgrade(&current_dir);
                    let new_dir = Directory::new(&output.name, Some(weak));
                    idk.subdirs.push(Rc::new(RefCell::new(new_dir)))
                }
                OutputType::Size(size) => {
                    let mut idk = current_dir.borrow_mut();
                    idk.files.push(SingleFile::new(&output.name, size))
                }
            },
        }
    }

    println!("root size: {:?}", root.borrow_mut().size());

    match part {
        Part::One => {
            let result = root.borrow_mut().sum_subdirs_with_size(100000);
            Ok(result)
        }
        Part::Two => {
            let unused = 70_000_000 - root.borrow_mut().size();
            println!("unused: {:?}", unused);
            let required = 30_000_000 - unused;
            let result = root.borrow_mut().smallest_subdir_with_size(required);
            Ok(result)
        }
    }
}

fn main() -> Result<()> {
    let start = Instant::now();

    let test1 = solve(Part::One, Input::Test)?;
    assert_eq!(test1, 95437);

    let result1 = solve(Part::One, Input::Puzzle)?;
    println!("part 1 result: {}", result1);

    let test2 = solve(Part::Two, Input::Test)?;
    assert_eq!(test2, 24933642);

    let result2 = solve(Part::Two, Input::Puzzle)?;
    println!("part 2 result: {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
